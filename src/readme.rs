//! Regenerate the generated sections of the root `readme.md`:
//! the dictionary table (between `<!--support start/end-->`) and the
//! fork-patch list (between `<!--patches start/end-->`).

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::patches::PATCHES;
use crate::table::{DICTIONARIES, FROZEN, Source};

pub fn run(root: &Path) -> Result<()> {
    let path = root.join("readme.md");
    let readme = fs::read_to_string(&path)?;

    let readme = replace_section(&readme, "support", &support_table())?;
    let readme = replace_section(&readme, "patches", &patch_list())?;

    fs::write(&path, readme)?;
    println!("readme: regenerated tables ✓");
    Ok(())
}

fn support_table() -> String {
    struct Row {
        code: &'static str,
        name: &'static str,
        spdx: &'static str,
        has_license: bool,
        source: Option<&'static Source>,
    }

    let mut rows: Vec<Row> = DICTIONARIES
        .iter()
        .map(|d| Row {
            code: d.code,
            name: d.name,
            spdx: d.spdx,
            has_license: d.license.is_some(),
            source: Source::by_name(d.source),
        })
        .chain(FROZEN.iter().map(|f| Row {
            code: f.code,
            name: f.name,
            spdx: f.spdx,
            has_license: f.has_license,
            source: None,
        }))
        .collect();
    rows.sort_by_key(|r| r.code);

    let mut out = String::new();
    out.push_str(&format!(
        "In total {} dictionaries are provided.\n\n",
        rows.len()
    ));
    out.push_str("| Code | Language | License | Source |\n");
    out.push_str("| - | - | - | - |\n");

    for row in rows {
        let license = if row.has_license {
            format!("[{}](dictionaries/{}/license)", row.spdx, row.code)
        } else {
            row.spdx.to_owned()
        };
        let source = match row.source {
            Some(s) => format!("[{}]({})", source_label(s.page), s.page),
            None => "(frozen — upstream gone)".to_owned(),
        };
        out.push_str(&format!(
            "| [`{code}`](dictionaries/{code}) | {name} | {license} | {source} |\n",
            code = row.code,
            name = row.name,
        ));
    }
    out
}

/// Short display label for a source page URL, e.g.
/// `https://github.com/wooorm/dictionaries` -> `wooorm/dictionaries`.
fn source_label(page: &str) -> String {
    let rest = page
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let (host, path) = rest.split_once('/').unwrap_or((rest, ""));
    let path = path.trim_end_matches('/');

    match host {
        "github.com" => path.to_owned(),
        "gitlab.com" => format!("gl:{path}"),
        "sites.google.com" => path.split('/').nth(1).unwrap_or(path).to_owned(),
        _ => host.trim_start_matches("www.").to_owned(),
    }
}

fn patch_list() -> String {
    let mut out = String::new();
    for patch in PATCHES {
        // Collapse the multi-line summary strings into single spaces.
        let summary = patch
            .summary
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!("*   **`{}`**: {}\n", patch.code, summary));
    }
    out
}

fn replace_section(readme: &str, name: &str, content: &str) -> Result<String> {
    let start_marker = format!("<!--{name} start-->");
    let end_marker = format!("<!--{name} end-->");
    let start = readme
        .find(&start_marker)
        .with_context(|| format!("readme.md is missing {start_marker}"))?
        + start_marker.len();
    let end = readme
        .find(&end_marker)
        .with_context(|| format!("readme.md is missing {end_marker}"))?;
    anyhow::ensure!(
        start <= end,
        "readme.md markers for `{name}` are out of order"
    );

    Ok(format!(
        "{}\n\n{}\n{}",
        &readme[..start],
        content,
        &readme[end..]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_labels() {
        assert_eq!(
            source_label("https://github.com/wooorm/dictionaries"),
            "wooorm/dictionaries"
        );
        assert_eq!(
            source_label("http://bgoffice.sourceforge.net"),
            "bgoffice.sourceforge.net"
        );
        assert_eq!(
            source_label("http://www.translatoblog.cz/hunspell/"),
            "translatoblog.cz"
        );
        assert_eq!(
            source_label("https://sites.google.com/site/araktransfer/home/spell-checkers"),
            "araktransfer"
        );
        assert_eq!(
            source_label("https://gitlab.com/taissou/hunspell-files-for-occitan-lengadocian"),
            "gl:taissou/hunspell-files-for-occitan-lengadocian"
        );
    }

    #[test]
    fn replace_section_roundtrip() {
        let doc = "a\n<!--x start-->\nold\n<!--x end-->\nb\n";
        let out = replace_section(doc, "x", "new\n").unwrap();
        // Content is framed by blank lines, matching the readme's style.
        assert_eq!(out, "a\n<!--x start-->\n\nnew\n\n<!--x end-->\nb\n");
    }
}
