//! Cross-platform config/data paths.

use std::path::PathBuf;

use directories::ProjectDirs;

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "noldee", "rnpkill-rs")
}

pub fn config_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.config_dir().to_path_buf())
}

pub fn data_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.data_dir().to_path_buf())
}

pub fn history_file() -> Option<PathBuf> {
    data_dir().map(|d| d.join("history.jsonl"))
}