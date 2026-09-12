//! Domain types.

use std::path::PathBuf;
use std::time::SystemTime;

/// A target folder detected in the filesystem.
#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub mtime: Option<SystemTime>,
}

/// A project directory that contains one or more targets.
#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub targets: Vec<Target>,
}

impl Project {
    /// Total size of all targets in this project.
    pub fn total_size(&self) -> u64 {
        self.targets.iter().map(|t| t.size_bytes).sum()
    }
}