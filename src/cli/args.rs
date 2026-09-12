//! Command-line argument definitions.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rnpkill-rs",
    version,
    about = "Clean your dev folders, fast.",
    long_about = "A modern alternative to npkill, written in Rust.\n\
                  Finds and deletes node_modules, venvs, and other heavy dev folders."
)]
pub struct Args {
    /// Root directory to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Limit recursion depth
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// Skip size measurement (instant listing)
    #[arg(long)]
    pub no_size: bool,

    /// Only show folders untouched for N (e.g. 90d, 6m, 1y)
    #[arg(long)]
    pub older_than: Option<String>,

    /// Show what would be deleted, without deleting
    #[arg(long)]
    pub dry_run: bool,

    /// Export a session report (.json or .csv)
    #[arg(long)]
    pub report: Option<PathBuf>,

    /// UI theme
    #[arg(long, default_value = "default")]
    pub theme: String,

    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long)]
    pub include_generic: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show the last 10 cleanups
    History,
    /// Show aggregate statistics
    Stats,
    /// Export the full cleanup history to CSV
    ExportHistory {
        /// Destination CSV file
        path: PathBuf,
    },
    /// Download and install the latest release
    Upgrade,
}
