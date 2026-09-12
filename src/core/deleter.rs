//! Safe directory deletion.

use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::core::models::Target;

/// Deletes a single target directory, returning the bytes freed.
///
/// Performs safety checks before deleting:
/// - The path must exist.
/// - The path must not be `/`, `~`, or the current working directory.
pub fn delete_target(target: &Target) -> Result<u64> {
    if !target.path.exists() {
        bail!("path does not exist: {}", target.path.display());
    }

    if !is_safe_to_delete(&target.path) {
        bail!("refusing to delete unsafe path: {}", target.path.display());
    }

    std::fs::remove_dir_all(&target.path)
        .with_context(|| format!("failed to delete {}", target.path.display()))?;

    Ok(target.size_bytes)
}

/// Deletes multiple targets sequentially.
///
/// Returns `(freed_bytes, error_count)`.
pub fn delete_targets(targets: &[Target]) -> (u64, usize) {
    let mut freed = 0;
    let mut errors = 0;

    for t in targets {
        match delete_target(t) {
            Ok(bytes) => freed += bytes,
            Err(e) => {
                eprintln!("✗ {} — {}", t.path.display(), e);
                errors += 1;
            }
        }
    }

    (freed, errors)
}

/// Checks if a path is safe to delete.
pub fn is_safe_to_delete(path: &Path) -> bool {
    // Never delete root ("/")
    if path.parent().is_none() {
        return false;
    }

    // Never delete the home directory itself
    if let Some(home) = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(std::path::PathBuf::from)
    {
        if path == home {
            return false;
        }
    }

    // Never delete the current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if path == cwd {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_root() {
        assert!(!is_safe_to_delete(Path::new("/")));
    }

    #[test]
    fn allows_normal_path() {
        assert!(is_safe_to_delete(Path::new("/tmp/foo/bar")));
    }
}