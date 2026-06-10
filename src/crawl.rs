//! Download upstream archives into `archive/` (a cache — delete a file to
//! re-download) and extract them into `source/`.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::table::{ArchiveKind, ExtraFile, Fetch, Source};

pub fn run(root: &Path, sources: &[&'static Source]) -> Result<()> {
    let client = client()?;
    let mut failed = Vec::new();

    for source in sources {
        match crawl_one(&client, root, source) {
            Ok(()) => println!("crawl: {} ✓", source.name),
            Err(e) => {
                eprintln!("crawl: {} ✗ {e:#}", source.name);
                failed.push(source.name);
            }
        }
    }

    if !failed.is_empty() {
        bail!("failed to crawl: {}", failed.join(", "));
    }
    Ok(())
}

fn client() -> Result<reqwest::blocking::Client> {
    // Some hosts (sites.google.com among them) reject requests without a
    // browser-looking user agent.
    Ok(reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) dictionaries-builder")
        .timeout(std::time::Duration::from_secs(120))
        .build()?)
}

fn crawl_one(client: &reqwest::blocking::Client, root: &Path, source: &Source) -> Result<()> {
    let source_dir = root.join("source").join(source.name);

    match &source.fetch {
        Fetch::Archive { url, kind } => {
            let archive =
                root.join("archive")
                    .join(format!("{}.{}", source.name, kind.extension()));
            if !archive.exists() {
                download(client, url, &archive)?;
            }
            if !source_dir.exists() {
                extract(&archive, *kind, &source_dir)
                    .with_context(|| format!("extracting {}", archive.display()))?;
            }
        }
        Fetch::Files(files) => {
            fetch_files(client, &source_dir, files)?;
        }
    }

    fetch_files(client, &source_dir, source.extra)
}

fn fetch_files(
    client: &reqwest::blocking::Client,
    source_dir: &Path,
    files: &[ExtraFile],
) -> Result<()> {
    for file in files {
        let dest = source_dir.join(file.dest);
        if !dest.exists() {
            download(client, file.url, &dest)?;
        }
    }
    Ok(())
}

/// Download to `<dest>.part`, then rename, so an interrupted download is
/// never mistaken for a cached archive.
fn download(client: &reqwest::blocking::Client, url: &str, dest: &Path) -> Result<()> {
    println!("  fetching {url}");
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let part = dest.with_extension("part");

    let mut response = client
        .get(url)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .with_context(|| format!("GET {url}"))?;

    let mut file = File::create(&part)?;
    std::io::copy(&mut response, &mut file).with_context(|| format!("downloading {url}"))?;
    file.flush()?;
    drop(file);

    fs::rename(&part, dest)?;
    Ok(())
}

impl ArchiveKind {
    fn extension(self) -> &'static str {
        match self {
            ArchiveKind::Zip => "zip",
            ArchiveKind::TarGz => "tar.gz",
            ArchiveKind::TarBz2 => "tar.bz2",
        }
    }
}

/// Extract to a temporary sibling directory, then rename into place, so an
/// interrupted extraction is never mistaken for a complete source.
fn extract(archive: &Path, kind: ArchiveKind, dest: &Path) -> Result<()> {
    let parent = dest.parent().context("destination has no parent")?;
    fs::create_dir_all(parent)?;
    let tmp = parent.join(format!(
        ".tmp-{}",
        dest.file_name().unwrap().to_string_lossy()
    ));
    if tmp.exists() {
        fs::remove_dir_all(&tmp)?;
    }
    fs::create_dir_all(&tmp)?;

    match kind {
        ArchiveKind::Zip => extract_zip(archive, &tmp)?,
        ArchiveKind::TarGz => {
            let file = File::open(archive)?;
            extract_tar_strip1(tar::Archive::new(flate2::read::GzDecoder::new(file)), &tmp)?;
        }
        ArchiveKind::TarBz2 => {
            let file = File::open(archive)?;
            extract_tar_strip1(tar::Archive::new(bzip2::read::BzDecoder::new(file)), &tmp)?;
        }
    }

    fs::rename(&tmp, dest)?;
    Ok(())
}

/// Plain zip extraction (no component stripping, like `unzip -d`). The zip
/// crate rejects path-traversal entries itself.
pub fn extract_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = File::open(archive).with_context(|| format!("opening {}", archive.display()))?;
    let mut zip = zip::ZipArchive::new(file)?;
    zip.extract(dest)?;
    Ok(())
}

/// Tar extraction emulating `--strip-components=1`, as crawl.sh used for all
/// tarballs. Entries that *are* the single top-level directory are skipped.
fn extract_tar_strip1<R: std::io::Read>(mut tar: tar::Archive<R>, dest: &Path) -> Result<()> {
    for entry in tar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();

        let mut components = path.components();
        match components.next() {
            Some(Component::Normal(_)) => {}
            _ => bail!("refusing to extract tar entry with unsafe path {path:?}"),
        }
        let stripped: PathBuf = components.collect();
        if stripped.as_os_str().is_empty() {
            continue;
        }
        if stripped
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
        {
            bail!("refusing to extract tar entry with unsafe path {path:?}");
        }

        let out = dest.join(&stripped);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        entry.unpack(&out)?;
    }
    Ok(())
}
