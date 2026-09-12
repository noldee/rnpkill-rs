//! Directory size measurement with parallel execution.

use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;

/// Measures a directory's total size (recursively) in bytes.
///
/// Ignores symlinks and permission errors, returning a lower bound
/// rather than aborting.
pub fn measure_dir(path: &Path) -> u64 {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

/// Measures many directories in parallel using rayon.
///
/// Returns a `Vec<u64>` in the **same order** as the input paths.
pub fn measure_many(paths: &[PathBuf]) -> Vec<u64> {
    paths.par_iter().map(|p| measure_dir(p)).collect()
}