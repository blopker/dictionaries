mod build;
mod crawl;
mod generate;
mod patches;
mod readme;
mod table;
mod validate;

use std::collections::BTreeSet;
use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use table::{DICTIONARIES, Dictionary, SOURCES, Source};

/// Build pipeline for the Codebook hunspell dictionary collection.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Download and extract upstream sources into `archive/` and `source/`
    Crawl(Filter),
    /// Run make/configure steps for sources that build their hunspell files
    Build(Filter),
    /// Normalize, patch, and write `dictionaries/<code>/` from `source/`
    Generate(Filter),
    /// Check that every dictionary parses with spellbook
    Validate(Filter),
    /// Apply registered patches to already-generated dictionaries in place
    /// (for newly added patches, without re-crawling sources)
    Patch(Filter),
    /// Regenerate the dictionary table in `readme.md`
    Readme,
    /// Crawl, build, generate, readme, and validate in one go
    All(Filter),
}

#[derive(clap::Args)]
struct Filter {
    /// Only process these dictionary codes (repeatable, e.g. --only uk --only da)
    #[arg(long)]
    only: Vec<String>,
}

impl Filter {
    fn is_all(&self) -> bool {
        self.only.is_empty()
    }

    fn dictionaries(&self) -> Result<Vec<&'static Dictionary>> {
        if self.is_all() {
            return Ok(DICTIONARIES.iter().collect());
        }
        self.only
            .iter()
            .map(|code| {
                Dictionary::by_code(code).ok_or_else(|| {
                    if table::FROZEN.iter().any(|f| f.code == code) {
                        anyhow::anyhow!(
                            "dictionary `{code}` is frozen (its upstream is gone); it is \
                             always validated but cannot be regenerated"
                        )
                    } else {
                        anyhow::anyhow!("unknown dictionary code `{code}`")
                    }
                })
            })
            .collect()
    }

    /// The sources needed for the selected dictionaries, in table order.
    fn sources(&self) -> Result<Vec<&'static Source>> {
        let names: BTreeSet<&str> = self.dictionaries()?.iter().map(|d| d.source).collect();
        Ok(SOURCES.iter().filter(|s| names.contains(s.name)).collect())
    }
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let root = root();

    match &cli.cmd {
        Cmd::Crawl(f) => crawl::run(root, &f.sources()?),
        Cmd::Build(f) => build::run(root, &f.sources()?),
        Cmd::Generate(f) => generate::run(root, &f.dictionaries()?, f.is_all()),
        Cmd::Validate(f) => validate::run(root, &f.dictionaries()?, f.is_all()),
        Cmd::Patch(f) => {
            anyhow::ensure!(
                !f.is_all(),
                "`patch` rewrites files in place; pass the codes explicitly with --only"
            );
            generate::patch_in_place(root, &f.dictionaries()?)
        }
        Cmd::Readme => readme::run(root),
        Cmd::All(f) => {
            crawl::run(root, &f.sources()?)?;
            build::run(root, &f.sources()?)?;
            generate::run(root, &f.dictionaries()?, f.is_all())?;
            readme::run(root)?;
            validate::run(root, &f.dictionaries()?, f.is_all())
        }
    }
}
