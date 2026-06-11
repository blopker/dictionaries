//! Turn crawled sources into `dictionaries/<code>/` output: decode to UTF-8,
//! normalize, apply fork patches, write, and prune stale files.
//!
//! Normalization is byte-for-byte faithful to the upstream shell pipeline
//! (`iconv | sed ...` in crawl.sh), so regenerating from an unchanged
//! upstream source produces no diff.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::patches::{self, DictText};
use crate::table::{DICTIONARIES, Dictionary, Enc, FROZEN, SourceFile};

pub fn run(root: &Path, dicts: &[&'static Dictionary], full_tree: bool) -> Result<()> {
    for d in dicts {
        generate_one(root, d).with_context(|| format!("generating `{}`", d.code))?;
        println!("generate: {} ✓", d.code);
    }
    if full_tree {
        check_no_stray_dirs(root)?;
    }
    Ok(())
}

fn generate_one(root: &Path, d: &Dictionary) -> Result<()> {
    let source_dir = root.join("source").join(d.source);
    if !source_dir.is_dir() {
        bail!(
            "source/{} does not exist — run `crawl` (and `build`) first",
            d.source
        );
    }

    let mut text = DictText {
        aff: normalize(&read_source(&source_dir, &d.aff)?, true),
        dic: normalize(&read_source(&source_dir, &d.dic)?, false),
    };
    patches::apply(d.code, &mut text)?;

    let out_dir = root.join("dictionaries").join(d.code);
    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join("index.aff"), &text.aff)?;
    fs::write(out_dir.join("index.dic"), &text.dic)?;

    let mut keep = vec!["index.aff", "index.dic"];
    if let Some(license) = &d.license {
        let license_text = normalize(&read_source(&source_dir, license)?, false);
        fs::write(out_dir.join("license"), license_text)?;
        keep.push("license");
    }

    prune(&out_dir, &keep)
}

/// Apply registered patches to already-generated dictionaries in place.
/// Useful when adding a new patch without re-crawling: patches fail loudly
/// when already applied, so this cannot double-apply.
pub fn patch_in_place(root: &Path, dicts: &[&'static Dictionary]) -> Result<()> {
    for d in dicts {
        let dir = root.join("dictionaries").join(d.code);
        let mut text = DictText {
            aff: fs::read_to_string(dir.join("index.aff"))?,
            dic: fs::read_to_string(dir.join("index.dic"))?,
        };
        patches::apply(d.code, &mut text).with_context(|| format!("patching `{}`", d.code))?;
        fs::write(dir.join("index.aff"), &text.aff)?;
        fs::write(dir.join("index.dic"), &text.dic)?;
        println!("patch: {} ✓", d.code);
    }
    Ok(())
}

fn read_source(source_dir: &Path, file: &SourceFile) -> Result<String> {
    let path = source_dir.join(file.path);
    let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
    decode(&bytes, file.encoding).with_context(|| format!("decoding {}", path.display()))
}

fn decode(bytes: &[u8], encoding: Enc) -> Result<String> {
    let decoder = match encoding {
        Enc::Utf8 => {
            return String::from_utf8(bytes.to_vec()).context("invalid UTF-8");
        }
        // Identity mapping byte -> code point. encoding_rs's "iso-8859-1"
        // label is WHATWG-aliased to windows-1252, which differs in
        // 0x80-0x9F, so it cannot be used here.
        Enc::Latin1 => {
            return Ok(bytes.iter().map(|&b| b as char).collect());
        }
        Enc::Iso8859_2 => encoding_rs::ISO_8859_2,
        Enc::Iso8859_15 => encoding_rs::ISO_8859_15,
        Enc::Cp1251 => encoding_rs::WINDOWS_1251,
        Enc::Cp1252 => encoding_rs::WINDOWS_1252,
    };
    match decoder.decode_without_bom_handling_and_without_replacement(bytes) {
        Some(text) => Ok(text.into_owned()),
        None => bail!("malformed {:?} byte sequence", encoding),
    }
}

/// Port of the upstream sed pipeline. Order matters and is preserved:
/// 1. (aff only) rewrite `SET ...` lines to `SET UTF-8`
/// 2. strip a UTF-8 BOM from the start
/// 3. per line: strip trailing spaces/tabs, then one trailing `\r`
///    (in that order — spaces before a `\r` survive, like the original seds)
/// 4. ensure the file ends with exactly its lines plus a final newline
fn normalize(text: &str, is_aff: bool) -> String {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);

    let lines: Vec<String> = text
        .split('\n')
        .map(|line| {
            if is_aff && line.starts_with("SET ") {
                return "SET UTF-8".to_owned();
            }
            let line = line.trim_end_matches([' ', '\t']);
            let line = line.strip_suffix('\r').unwrap_or(line);
            line.to_owned()
        })
        .collect();

    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn prune(dir: &Path, keep: &[&str]) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        if !keep.iter().any(|k| name.as_os_str() == *k) {
            fs::remove_file(entry.path())
                .with_context(|| format!("pruning {}", entry.path().display()))?;
            println!("  pruned {}", entry.path().display());
        }
    }
    Ok(())
}

/// Every directory under `dictionaries/` must be declared in the table.
fn check_no_stray_dirs(root: &Path) -> Result<()> {
    for entry in fs::read_dir(root.join("dictionaries"))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type()?.is_dir()
            && !name.starts_with('.')
            && !DICTIONARIES.iter().any(|d| d.code == name)
            && !FROZEN.iter().any(|f| f.code == name)
        {
            bail!("dictionaries/{name} exists but is not in the table — remove it or add an entry");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_is_faithful_to_sed_pipeline() {
        // BOM stripped, CR removed. Spaces and tabs *before* a \r survive,
        // exactly like the original pipeline (sed strips [ \t]*$ before \r$,
        // and \r blocks the end-of-line anchor).
        assert_eq!(normalize("\u{feff}a \r\nb\t\r\n", false), "a \nb\t\n");
        // Plain trailing spaces (no \r) are stripped.
        assert_eq!(normalize("word\nplain  \n", false), "word\nplain\n");
        // Final newline added when missing; existing blank lines kept.
        assert_eq!(normalize("a\n\nb", false), "a\n\nb\n");
        assert_eq!(normalize("a\n\n\n", false), "a\n\n\n");
    }

    #[test]
    fn normalize_rewrites_set_line_in_aff_only() {
        assert_eq!(
            normalize("SET ISO8859-1\nTRY abc\n", true),
            "SET UTF-8\nTRY abc\n"
        );
        // CR after the SET value is consumed by the rewrite, like sed's `.*`.
        assert_eq!(
            normalize("SET ISO8859-1\r\nTRY abc\r\n", true),
            "SET UTF-8\nTRY abc\n"
        );
        // dic files keep SET-looking lines untouched.
        assert_eq!(normalize("SET ISO8859-1\n", false), "SET ISO8859-1\n");
    }

    #[test]
    fn latin1_decode_is_not_cp1252() {
        // 0x80-0x9F are C1 controls in ISO 8859-1; cp1252 would map 0x80 to €.
        assert_eq!(decode(&[0x80], Enc::Latin1).unwrap(), "\u{80}");
        assert_eq!(decode(&[0x80], Enc::Cp1252).unwrap(), "€");
        // 0xE9 is é in both.
        assert_eq!(decode(&[0xE9], Enc::Latin1).unwrap(), "é");
    }

    #[test]
    fn utf8_decode_rejects_invalid() {
        assert!(decode(&[0xC3], Enc::Utf8).is_err());
    }
}
