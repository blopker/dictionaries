//! Validate that every dictionary in `dictionaries/` parses with spellbook —
//! the hunspell implementation Codebook uses — and that the directory tree
//! matches the table exactly.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use rayon::prelude::*;

use crate::table::{Dictionary, FROZEN, SOURCES};

pub fn run(root: &Path, dicts: &[&'static Dictionary], full_tree: bool) -> Result<()> {
    // (code, license expected) for the selected dictionaries; frozen
    // dictionaries are always included — they're cheap and have no other
    // pipeline step that exercises them.
    let mut targets: Vec<(&str, bool)> = dicts
        .iter()
        .map(|d| (d.code, d.license.is_some()))
        .collect();
    targets.extend(FROZEN.iter().map(|f| (f.code, f.has_license)));

    let mut failures: Vec<String> = targets
        .par_iter()
        .filter_map(|&(code, has_license)| {
            validate_one(root, code, has_license)
                .err()
                .map(|e| format!("{code}: {e:#}"))
        })
        .collect();

    if full_tree && let Err(e) = check_tree(root) {
        failures.push(format!("{e:#}"));
    }

    if failures.is_empty() {
        println!("validate: {} dictionaries OK", targets.len());
        Ok(())
    } else {
        failures.sort();
        for failure in &failures {
            eprintln!("✗ {failure}");
        }
        bail!(
            "{} of {} dictionaries failed validation",
            failures.len(),
            targets.len()
        );
    }
}

fn validate_one(root: &Path, code: &str, has_license: bool) -> Result<()> {
    let dir = root.join("dictionaries").join(code);
    let aff = read_utf8(&dir.join("index.aff"))?;
    let dic = read_utf8(&dir.join("index.dic"))?;

    spellbook::Dictionary::new(&aff, &dic)
        .map_err(|e| anyhow::anyhow!("spellbook failed to parse: {e}"))?;

    match (has_license, dir.join("license").exists()) {
        (true, false) => bail!("license file missing but table declares one"),
        (false, true) => bail!("license file present but table declares none"),
        _ => {}
    }
    Ok(())
}

fn read_utf8(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    String::from_utf8(bytes).with_context(|| format!("{} is not valid UTF-8", path.display()))
}

/// `dictionaries/` must contain exactly the table's codes (regenerable and
/// frozen), and every dictionary must reference a declared source.
fn check_tree(root: &Path) -> Result<()> {
    let mut expected: BTreeSet<&str> = crate::table::DICTIONARIES.iter().map(|d| d.code).collect();
    expected.extend(FROZEN.iter().map(|f| f.code));

    for d in crate::table::DICTIONARIES {
        if !SOURCES.iter().any(|s| s.name == d.source) {
            bail!(
                "dictionary `{}` references unknown source `{}`",
                d.code,
                d.source
            );
        }
    }

    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(root.join("dictionaries"))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type()?.is_dir() && !name.starts_with('.') {
            actual.insert(name);
        }
    }

    let actual_refs: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    let missing: Vec<_> = expected.difference(&actual_refs).collect();
    let unknown: Vec<_> = actual_refs.difference(&expected).collect();
    if !missing.is_empty() || !unknown.is_empty() {
        bail!(
            "dictionaries/ does not match table — missing: {missing:?}, not in table: {unknown:?}"
        );
    }
    Ok(())
}
