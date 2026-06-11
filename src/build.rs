//! Build steps for sources that don't ship ready-made hunspell files.
//! Requires `make` (and for hebrew: `perl` and a C toolchain) on the host.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::table::{Build, Source};

pub fn run(root: &Path, sources: &[&'static Source]) -> Result<()> {
    for source in sources {
        let Some(build) = &source.build else { continue };
        let source_dir = root.join("source").join(source.name);
        if !source_dir.is_dir() {
            bail!("source/{} does not exist — run `crawl` first", source.name);
        }
        build_one(build, &source_dir).with_context(|| format!("building `{}`", source.name))?;
        println!("build: {} ✓", source.name);
    }
    Ok(())
}

fn build_one(build: &Build, source_dir: &Path) -> Result<()> {
    match build {
        Build::Make { subdir, targets } => make(&source_dir.join(subdir), targets, false),
        Build::ConfigureMake { subdir, targets } => {
            let dir = source_dir.join(subdir);
            if !dir.join("Makefile").exists() {
                run_command(Command::new("./configure").current_dir(&dir))?;
            }
            make(&dir, targets, true)
        }
        Build::NestedZips { zips } => {
            for (zip, dest) in *zips {
                let dest = source_dir.join(dest);
                if !dest.exists() {
                    crate::crawl::extract_zip(&source_dir.join(zip), &dest)?;
                }
            }
            Ok(())
        }
    }
}

fn make(dir: &Path, targets: &[&str], with_perl5lib: bool) -> Result<()> {
    let mut command = Command::new("make");
    command.current_dir(dir).args(targets);
    if with_perl5lib {
        // The hebrew build requires the source dir itself on perl's path.
        let existing = std::env::var("PERL5LIB").unwrap_or_default();
        command.env("PERL5LIB", format!("{existing}:."));
    }
    run_command(&mut command)
}

fn run_command(command: &mut Command) -> Result<()> {
    let rendered = format!("{command:?}");
    let status = command
        .status()
        .with_context(|| format!("spawning {rendered}"))?;
    if !status.success() {
        bail!("{rendered} exited with {status}");
    }
    Ok(())
}
