//! File walker used by the `parfit` binary when path arguments
//! are supplied. Respects `.gitignore` by default via the
//! `ignore` crate (the same walker ripgrep and fd use).

use std::io;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;

/// Collect the list of files to reflow, given the user's path
/// arguments and filter flags.
pub fn walk(
    paths: &[PathBuf],
    recursive: bool,
    includes: &[String],
    excludes: &[String],
) -> io::Result<Vec<PathBuf>> {
    let include_set = build_globset(includes)?;
    let exclude_set = build_globset(excludes)?;
    let mut out = Vec::new();

    for path in paths {
        if path.is_file() {
            if keep(path, include_set.as_ref(), exclude_set.as_ref()) {
                out.push(path.clone());
            }
            continue;
        }
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{}: no such file or directory", path.display()),
            ));
        }
        if !recursive {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "{} is a directory; pass --recursive to walk it",
                    path.display()
                ),
            ));
        }

        let walker = WalkBuilder::new(path).build();
        for entry in walker {
            let entry = entry
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }
            let p = entry.path();
            if keep(p, include_set.as_ref(), exclude_set.as_ref()) {
                out.push(p.to_path_buf());
            }
        }
    }

    Ok(out)
}

fn build_globset(patterns: &[String]) -> io::Result<Option<GlobSet>> {
    if patterns.is_empty() {
        return Ok(None);
    }
    let mut builder = GlobSetBuilder::new();
    for p in patterns {
        let glob = Glob::new(p)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        builder.add(glob);
    }
    builder
        .build()
        .map(Some)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))
}

fn keep(path: &Path, include: Option<&GlobSet>, exclude: Option<&GlobSet>) -> bool {
    let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if let Some(ex) = exclude {
        if ex.is_match(fname) || ex.is_match(path) {
            return false;
        }
    }
    if let Some(inc) = include {
        return inc.is_match(fname) || inc.is_match(path);
    }
    true
}
