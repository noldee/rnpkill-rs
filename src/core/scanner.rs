//! Filesystem scanner.
//!
//! Walks a directory tree and detects target folders (node_modules,
//! venv, target, vendor, etc.).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::core::models::{Project, Target};

/// Carpetas seguras de detectar por defecto — casi nunca son código
/// escrito a mano ni output de producción.
pub const SAFE_TARGETS: &[&str] = &[
    // JavaScript / Node
    "node_modules",
    ".next",
    ".nuxt",
    ".parcel-cache",
    ".turbo",
    ".svelte-kit",
    // Python
    "venv",
    ".venv",
    "env",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".tox",
    // Rust / Java / Go / PHP
    "target",
    ".gradle",
    ".m2",
    "vendor",
    // Swift / Dart / Elixir / Terraform
    ".build",
    "DerivedData",
    ".dart_tool",
    "_build",
    "deps",
    ".terraform",
];

/// Nombres de carpeta ambiguos: en algunos ecosistemas son build
/// output, pero en otros contienen código propio o assets de
/// producción (ej. un `dist/` servido en prod, un `bin/` con scripts
/// del repo). Nunca se activan por defecto — solo si el usuario
/// pasa `--include-generic`.
pub const GENERIC_TARGETS: &[&str] = &["dist", "build", "bin", "obj", "out"];

pub struct Scanner {
    targets: HashSet<String>,
    max_depth: Option<usize>,
}

impl Scanner {
    /// Scanner con el set seguro por defecto (sin dist/build/bin/obj/out).
    pub fn new(max_depth: Option<usize>) -> Self {
        Self::with_options(max_depth, false)
    }

    /// Scanner configurable: `include_generic` suma dist/build/bin/obj/out.
    pub fn with_options(max_depth: Option<usize>, include_generic: bool) -> Self {
        let mut targets: HashSet<String> = SAFE_TARGETS.iter().map(|s| s.to_string()).collect();
        if include_generic {
            targets.extend(GENERIC_TARGETS.iter().map(|s| s.to_string()));
        }
        Self { targets, max_depth }
    }

    /// Scans `root` and returns the projects with targets.
    ///
    /// Uses a manual pruning pass instead of `filter_entry`: once a
    /// target directory is found, its descendants are skipped, which
    /// is critical for performance (node_modules can have hundreds of
    /// thousands of files).
    pub fn scan(&self, root: &Path) -> Result<Vec<Project>, std::io::Error> {
        let max_depth = self.max_depth.unwrap_or(6);
        let mut groups: HashMap<PathBuf, Vec<Target>> = HashMap::new();
        let mut pruned: Vec<PathBuf> = Vec::new();

        let all_entries: Vec<_> = WalkDir::new(root)
            .max_depth(max_depth)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        for entry in all_entries {
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
            pruned.push(path);
        }

        Ok(groups
            .into_iter()
            .map(|(path, targets)| Project { path, targets })
            .collect())
    }
}
