//! Filesystem scanner.
//!
//! Walks a directory tree and detects target folders (node_modules,
//! venv, target, vendor, etc.).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::core::models::{Project, Target};

/// Folders that rnpkill-rs looks for, out of the box.
pub const DEFAULT_TARGETS: &[&str] = &[
    // JavaScript / Node
    "node_modules", ".next", ".nuxt", ".parcel-cache", ".turbo", ".svelte-kit",
    // Python
    "venv", ".venv", "env", "__pycache__",
    ".pytest_cache", ".mypy_cache", ".ruff_cache", ".tox",
    // Rust / Java / Go / PHP
    "target", ".gradle", ".m2", "vendor",
    // Generic bundlers
    "dist", "build",
];

pub struct Scanner {
    targets: HashSet<String>,
    max_depth: Option<usize>,
}

impl Scanner {
    /// Creates a scanner with the default target set.
    pub fn new(max_depth: Option<usize>) -> Self {
        Self {
            targets: DEFAULT_TARGETS.iter().map(|s| s.to_string()).collect(),
            max_depth,
        }
    }

    /// Scans `root` and returns the projects with targets.
    ///
    /// Uses `filter_entry` to avoid descending into detected targets,
    /// which is critical for performance (node_modules can have
    /// hundreds of thousands of files).
          /// Scans `root` and returns the projects with targets.
    ///
    /// Uses `filter_entry` to avoid descending into detected targets,
    /// which is critical for performance (node_modules can have
    /// hundreds of thousands of files).
    ///
    /// The filter function returns `true` for target dirs (so we SEE
    /// them) but their children get filtered out because we check
    /// if ANY ancestor was a target. Simpler: use the entry's depth
    /// info together with a manual pruning pass.
        pub fn scan(&self, root: &Path) -> Result<Vec<Project>, std::io::Error> {
        let max_depth = self.max_depth.unwrap_or(6);
        let mut groups: HashMap<PathBuf, Vec<Target>> = HashMap::new();
        let mut pruned: Vec<PathBuf> = Vec::new();

        // Paso 1: recolectar todos los entries primero (sin closure mutable)
        let all_entries: Vec<_> = WalkDir::new(root)
            .max_depth(max_depth)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        // Paso 2: procesar y podar en memoria
        for entry in all_entries {
            // Skip si está dentro de algo podado
            if pruned.iter().any(|skip| entry.path().starts_with(skip)) {
                continue;
            }

            if !entry.file_type().is_dir() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if !self.targets.contains(&name) {
                continue;
            }

            let path = entry.path().to_path_buf();
            let parent = match path.parent() {
                Some(p) => p.to_path_buf(),
                None => continue,
            };

            let target = Target {
                name,
                path: path.clone(),
                size_bytes: 0,
                mtime: entry.metadata().ok().and_then(|m| m.modified().ok()),
            };

            groups.entry(parent).or_default().push(target);

            // Podar: registramos la ruta para saltarnos sus descendientes
            pruned.push(path);
        }

        Ok(groups
            .into_iter()
            .map(|(path, targets)| Project { path, targets })
            .collect())
    }
}