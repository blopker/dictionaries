//! Fork-local modifications applied to specific dictionaries during
//! `generate`, after normalization and before writing.
//!
//! Patches run on freshly normalized upstream text. Each one asserts the
//! exact text it expects and fails loudly when upstream changes, so a patch
//! can never silently stop applying (or apply twice).

use anyhow::{Context, Result, bail, ensure};

pub struct DictText {
    pub aff: String,
    pub dic: String,
}

pub struct Patch {
    pub code: &'static str,
    /// One-line summary, surfaced in the readme.
    pub summary: &'static str,
    pub apply: fn(&mut DictText) -> Result<()>,
}

pub static PATCHES: &[Patch] = &[
    Patch {
        code: "uk",
        summary: "remove ICONV rules that mapped every Latin letter to the digit 0, \
                  which made hunspell silently accept all Latin-script words as numbers",
        apply: patch_uk,
    },
    Patch {
        code: "da",
        summary: "escape `/` in three slash-containing entries (quoting is not valid \
                  dic syntax) and fix a corrupted FedEx/Fedkrog line",
        apply: patch_da,
    },
    Patch {
        code: "el-polyton",
        summary: "replace a tab with a space inside a REP rule (hunspell treats the \
                  tab as a field separator, breaking the rule)",
        apply: patch_el_polyton,
    },
    Patch {
        code: "br",
        summary: "fix three entries flagged `m01` — with FLAG long that is one-and-a-half \
                  flags; the defined flag `m0` is meant",
        apply: patch_br,
    },
    Patch {
        code: "gl",
        summary: "fix a corrupted numeric flag `2iñer30` (stray text spliced into `230`, \
                  the present-tense suffix flag) on the entry for `tumbar`",
        apply: patch_gl,
    },
    Patch {
        code: "hy",
        summary: "fix the `SFX VD` block header declaring 171 rules when 172 follow",
        apply: patch_hy,
    },
    Patch {
        code: "ia",
        summary: "remove a doubled closing bracket (`[oiyu]]`) from seven PFX conditions",
        apply: patch_ia,
    },
    Patch {
        code: "la",
        summary: "fix two affix rules misspelled `SFK` instead of `SFX`, which also broke \
                  the surrounding block's declared rule count",
        apply: patch_la,
    },
    Patch {
        code: "mn",
        summary: "remove the COMPOUNDRULE block — its `[a0,a1,...]` alternation syntax is \
                  not valid hunspell and no parser accepts it",
        apply: patch_mn,
    },
    Patch {
        code: "ne",
        summary: "strip a stray `X` from 171 numeric continuation flags (`17X` → `17`); \
                  the X-less flags are the ones actually defined, matching hunspell's \
                  lenient numeric parsing; also fix three corrupted entries (a slash \
                  inside a word, a stray `I` in a flag list, and two lines merged \
                  into one)",
        apply: patch_ne,
    },
    Patch {
        code: "sa",
        summary: "clean up stray carriage returns inside lines: repair dic entries and \
                  affix rules where a CR is glued onto a word, and drop single-rule \
                  affix blocks whose rule has a bare CR as a whole field (dead rules \
                  hunspell could never match)",
        apply: patch_sa,
    },
    Patch {
        code: "tr",
        summary: "renumber affix flag `0` to `9999` — hunspell numeric flags are defined \
                  as 1–65000 and flag 0 is rejected by spellbook",
        apply: patch_tr,
    },
];

pub fn apply(code: &str, text: &mut DictText) -> Result<()> {
    for patch in PATCHES.iter().filter(|p| p.code == code) {
        (patch.apply)(text).with_context(|| format!("patch `{code}` failed"))?;
    }
    Ok(())
}

/// Upstream uk maps every Latin letter (and some accented variants) to `0`
/// via ICONV, so any Latin-script word becomes a digit string — which
/// hunspell always accepts. Drop those rules, keeping only the two
/// apostrophe-normalization rules, and fix the rule count.
fn patch_uk(text: &mut DictText) -> Result<()> {
    const COUNT_OLD: &str = "ICONV 64";
    const COUNT_NEW: &str = "ICONV 2";
    const KEEP: [&str; 2] = ["ICONV ʼ '", "ICONV ’ '"];

    ensure!(
        text.aff.lines().any(|l| l == COUNT_OLD),
        "expected `{COUNT_OLD}` count line not found — upstream uk changed, re-audit this patch"
    );

    let mut removed = 0usize;
    let mut kept_rules = Vec::new();
    let mut out = Vec::new();
    for line in text.aff.lines() {
        if line == COUNT_OLD {
            out.push(COUNT_NEW);
            continue;
        }
        if let Some(rest) = line.strip_prefix("ICONV ")
            && !rest.starts_with(|c: char| c.is_ascii_digit())
        {
            // A conversion rule, not a count line.
            let mut chars = rest.chars();
            let from = chars.next();
            let is_latin_to_zero = from.is_some() && chars.as_str() == " 0";
            if is_latin_to_zero {
                removed += 1;
                continue;
            }
            kept_rules.push(line);
        }
        out.push(line);
    }

    ensure!(
        removed == 62,
        "expected to remove exactly 62 `ICONV <char> 0` rules, removed {removed}"
    );
    ensure!(
        kept_rules == KEEP,
        "expected the surviving ICONV rules to be the apostrophe rules {KEEP:?}, got {kept_rules:?}"
    );

    text.aff = join_lines(&out);
    Ok(())
}

/// Upstream da wraps a few `/`-containing words in double quotes, which is
/// not valid dic syntax (hunspell expects `\/` escapes), and one entry is
/// corrupted by a quoted FedEx fragment glued onto the following word.
fn patch_da(text: &mut DictText) -> Result<()> {
    text.dic = replace_line(&text.dic, "\"A/S\"", "A\\/S")?;
    text.dic = replace_line(&text.dic, "\"c/o\"", "c\\/o")?;
    text.dic = replace_line(&text.dic, "\"I/S\"", "I\\/S")?;
    text.dic = replace_line(
        &text.dic,
        "\"FedEx/9 ph:https://denstoredanske.lex.dk/FedEx\"Fedkrog/54,9",
        "Fedkrog/54,9",
    )?;
    Ok(())
}

/// Upstream el-polyton has a tab inside a REP rule where hunspell expects a
/// space (ported from a sed fix in upstream's crawl.sh).
fn patch_el_polyton(text: &mut DictText) -> Result<()> {
    text.aff = replace_line(&text.aff, "REP έψ\tεύσ", "REP έψ εύσ")?;
    Ok(())
}

fn patch_br(text: &mut DictText) -> Result<()> {
    text.dic = replace_count(&text.dic, "/m01 ", "/m0 ", 3)?;
    Ok(())
}

fn patch_gl(text: &mut DictText) -> Result<()> {
    text.dic = replace_count(&text.dic, ",2iñer30,", ",230,", 1)?;
    Ok(())
}

fn patch_hy(text: &mut DictText) -> Result<()> {
    text.aff = replace_line(&text.aff, "SFX\tVD\tY\t171", "SFX\tVD\tY\t172")?;
    Ok(())
}

fn patch_ia(text: &mut DictText) -> Result<()> {
    text.aff = replace_count(&text.aff, " [oiyu]]\n", " [oiyu]\n", 7)?;
    Ok(())
}

fn patch_la(text: &mut DictText) -> Result<()> {
    let (aff, changed) = edit_lines(&text.aff, |line| {
        line.strip_prefix("SFK ").map(|rest| format!("SFX {rest}"))
    });
    ensure!(
        changed == 2,
        "expected exactly 2 `SFK` lines, found {changed}"
    );
    text.aff = aff;
    Ok(())
}

fn patch_mn(text: &mut DictText) -> Result<()> {
    const BLOCK: [&str; 9] = [
        "COMPOUNDRULE 8",
        "COMPOUNDRULE (nn)*[a0,a1,a2,a3]",
        "COMPOUNDRULE (nn)*[e0,e1,e2,e3]",
        "COMPOUNDRULE (nn)*[i0,i1,i2,i3]",
        "COMPOUNDRULE (nn)*[g0,g1,g2,g3]",
        "COMPOUNDRULE (nn)*[m0,m1,m2,m3,p0,p1,p2,p3]",
        "COMPOUNDRULE (nn)*[o0,o1,o2,o3]",
        "COMPOUNDRULE (nn)*%?",
        "COMPOUNDRULE (nn)*.(nn)*%?",
    ];
    let mut seen = [0usize; BLOCK.len()];
    let lines: Vec<&str> = text
        .aff
        .lines()
        .filter(|line| match BLOCK.iter().position(|b| b == line) {
            Some(i) => {
                seen[i] += 1;
                false
            }
            None => true,
        })
        .collect();
    ensure!(
        seen.iter().all(|&n| n == 1),
        "expected each of the 9 COMPOUNDRULE block lines exactly once, matched {seen:?}"
    );
    text.aff = join_lines(&lines);
    Ok(())
}

fn patch_ne(text: &mut DictText) -> Result<()> {
    let mut removed = 0usize;
    let mut out = String::with_capacity(text.aff.len());
    let mut previous = '\0';
    for c in text.aff.chars() {
        if c == 'X' && previous.is_ascii_digit() {
            removed += 1;
        } else {
            out.push(c);
        }
        previous = c;
    }
    ensure!(
        removed == 171,
        "expected to strip exactly 171 digit-suffix `X` characters, stripped {removed}"
    );
    text.aff = out;
    // The previous line already covers `चलन`; this one means the variant `चल्ती`.
    text.dic = replace_line(&text.dic, "चलन/चल्ती/15,22", "चल्ती/15,22")?;
    text.dic = replace_line(&text.dic, "निजामती/I15,22", "निजामती/15,22")?;
    // Two entries separated by a bare carriage return instead of a newline.
    text.dic = replace_line(
        &text.dic,
        "महादेश/18,22,15,34\rमहादैया",
        "महादेश/18,22,15,34\nमहादैया",
    )?;
    Ok(())
}

/// Upstream sa has stray carriage returns *inside* lines (line-ending
/// conversion damage; trailing CRs are already handled by normalization).
/// Three cases:
/// - dic entries `word\r/flags`: the CR is glued to the word — remove it.
/// - affix rules with a CR glued into a field: remove it.
/// - affix rules with a bare CR as a whole field (strip/condition): the
///   original field is unrecoverable and the rule is dead in hunspell too
///   (no real word matches a CR); every such rule is the sole rule of a
///   `Y 1` block, so drop the rule together with its header.
fn patch_sa(text: &mut DictText) -> Result<()> {
    const REPAIRED_DIC: usize = 109;
    const REPAIRED_AFF: usize = 6;
    const DROPPED_RULES: usize = 119;

    let mut repaired_dic = 0usize;
    let (dic, _) = edit_lines(&text.dic, |line| {
        line.contains('\r').then(|| {
            repaired_dic += 1;
            line.replace('\r', "")
        })
    });
    ensure!(
        repaired_dic == REPAIRED_DIC,
        "expected to repair {REPAIRED_DIC} dic entries, repaired {repaired_dic}"
    );
    text.dic = dic;

    // Pass 1: classify affix lines and collect the flags of dropped rules.
    let mut dropped_flags = Vec::new();
    let mut repaired_aff = 0usize;
    for line in text.aff.lines() {
        if !line.contains('\r') {
            continue;
        }
        let fields: Vec<&str> = line.split(' ').collect();
        if fields.contains(&"\r") {
            ensure!(
                fields.len() >= 2 && (fields[0] == "SFX" || fields[0] == "PFX"),
                "unexpected bare-CR field outside an affix rule: {line:?}"
            );
            dropped_flags.push((fields[0].to_owned(), fields[1].to_owned()));
        } else {
            repaired_aff += 1;
        }
    }
    ensure!(
        dropped_flags.len() == DROPPED_RULES,
        "expected to drop {DROPPED_RULES} dead affix rules, found {}",
        dropped_flags.len()
    );
    ensure!(
        repaired_aff == REPAIRED_AFF,
        "expected to repair {REPAIRED_AFF} affix rules, repaired {repaired_aff}"
    );

    // Pass 2: every dropped rule must be the sole rule of its block; drop the
    // rule and its `<kw> <flag> Y 1` header, repair the rest.
    let mut lines = Vec::new();
    let mut dropped_headers = 0usize;
    for line in text.aff.lines() {
        let fields: Vec<&str> = line.split(' ').collect();
        let block = dropped_flags
            .iter()
            .any(|(kw, flag)| fields.len() >= 2 && fields[0] == kw && fields[1] == flag);
        if block {
            if fields.contains(&"\r") {
                continue; // the dead rule
            }
            ensure!(
                fields.len() == 4 && fields[2] == "Y" && fields[3] == "1",
                "dropped flag has another rule or a non-`Y 1` header: {line:?}"
            );
            dropped_headers += 1;
            continue;
        }
        if line.contains('\r') {
            lines.push(line.replace('\r', ""));
        } else {
            lines.push(line.to_owned());
        }
    }
    ensure!(
        dropped_headers == DROPPED_RULES,
        "expected to drop {DROPPED_RULES} block headers, dropped {dropped_headers}"
    );
    let line_refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    text.aff = join_lines(&line_refs);
    Ok(())
}

fn patch_tr(text: &mut DictText) -> Result<()> {
    text.aff = replace_line(&text.aff, "SFX 0 N 1", "SFX 9999 N 1")?;
    text.aff = replace_line(&text.aff, "SFX 0 0 de .", "SFX 9999 0 de .")?;

    // Replace the flag *token* `0` in every dic entry's flag list
    // (`word/flags morph...`, numeric flags comma-separated).
    let mut changed = 0usize;
    let (dic, _) = edit_lines(&text.dic, |line| {
        let slash = line.find('/')?;
        let after = &line[slash + 1..];
        let flags_end = after
            .find(|c: char| c.is_whitespace())
            .unwrap_or(after.len());
        let (flags, rest) = after.split_at(flags_end);
        if !flags.split(',').any(|f| f == "0") {
            return None;
        }
        changed += 1;
        let new_flags: Vec<&str> = flags
            .split(',')
            .map(|f| if f == "0" { "9999" } else { f })
            .collect();
        Some(format!(
            "{}/{}{}",
            &line[..slash],
            new_flags.join(","),
            rest
        ))
    });
    ensure!(
        changed == 4660,
        "expected exactly 4660 dic entries flagged `0`, found {changed}"
    );
    text.dic = dic;
    Ok(())
}

/// Replace a substring that must occur an exact number of times.
fn replace_count(text: &str, from: &str, to: &str, expected: usize) -> Result<String> {
    let count = text.matches(from).count();
    if count != expected {
        bail!(
            "expected `{from}` exactly {expected} times, found {count} — upstream changed, re-audit this patch"
        );
    }
    Ok(text.replace(from, to))
}

/// Rewrite lines through a callback; returns the new text and the number of
/// rewritten lines.
fn edit_lines(text: &str, mut f: impl FnMut(&str) -> Option<String>) -> (String, usize) {
    let mut changed = 0usize;
    let lines: Vec<String> = text
        .lines()
        .map(|line| match f(line) {
            Some(new) => {
                changed += 1;
                new
            }
            None => line.to_owned(),
        })
        .collect();
    let mut out = lines.join("\n");
    out.push('\n');
    (out, changed)
}

/// Replace a whole line that must occur exactly once.
fn replace_line(text: &str, from: &str, to: &str) -> Result<String> {
    let count = text.lines().filter(|l| *l == from).count();
    if count != 1 {
        bail!(
            "expected line `{from}` exactly once, found {count} times — upstream changed, re-audit this patch"
        );
    }
    let lines: Vec<&str> = text
        .lines()
        .map(|l| if l == from { to } else { l })
        .collect();
    Ok(join_lines(&lines))
}

fn join_lines(lines: &[&str]) -> String {
    let mut s = lines.join("\n");
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    /// Text of a repo file at a given git revision.
    fn git_show(spec: &str) -> String {
        let out = Command::new("git")
            .args(["show", spec])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("running git show");
        assert!(out.status.success(), "git show {spec} failed");
        String::from_utf8(out.stdout).expect("git show output not UTF-8")
    }

    /// The uk patch, applied to the upstream text from just before the manual
    /// fix landed (commit 5f81d73), must reproduce that commit exactly.
    #[test]
    fn uk_patch_matches_manual_fix() {
        let mut text = DictText {
            aff: git_show("5f81d73^:dictionaries/uk/index.aff"),
            dic: String::new(),
        };
        apply("uk", &mut text).expect("uk patch should apply to pre-fix upstream text");
        assert_eq!(text.aff, git_show("5f81d73:dictionaries/uk/index.aff"));
    }

    /// Same for the da patch (manual fix landed in commit 538ed7c).
    #[test]
    fn da_patch_matches_manual_fix() {
        let mut text = DictText {
            aff: String::new(),
            dic: git_show("538ed7c^:dictionaries/da/index.dic"),
        };
        apply("da", &mut text).expect("da patch should apply to pre-fix upstream text");
        assert_eq!(text.dic, git_show("538ed7c:dictionaries/da/index.dic"));
    }

    #[test]
    fn el_polyton_patch() {
        let mut text = DictText {
            aff: "REP 3\nREP έψ\tεύσ\nREP x y\n".into(),
            dic: String::new(),
        };
        apply("el-polyton", &mut text).expect("patch applies");
        assert_eq!(text.aff, "REP 3\nREP έψ εύσ\nREP x y\n");
    }

    /// Patches must fail loudly when their target text is absent.
    #[test]
    fn patches_fail_loudly_when_already_applied() {
        let mut patched = DictText {
            aff: git_show("5f81d73:dictionaries/uk/index.aff"),
            dic: String::new(),
        };
        assert!(apply("uk", &mut patched).is_err());

        let mut missing = DictText {
            aff: String::new(),
            dic: "Fedkrog/54,9\n".into(),
        };
        assert!(apply("da", &mut missing).is_err());
    }

    /// Codes without patches pass through untouched.
    #[test]
    fn unpatched_code_is_noop() {
        let mut text = DictText {
            aff: "SET UTF-8\n".into(),
            dic: "1\nword\n".into(),
        };
        apply("en", &mut text).expect("no-op");
        assert_eq!(text.aff, "SET UTF-8\n");
        assert_eq!(text.dic, "1\nword\n");
    }
}
