//! Entry point of rnpkill-rs.

use anyhow::Result;
use clap::Parser;

use rnpkill_rs::cli::args::Args;
use rnpkill_rs::cli::commands;

fn main() -> Result<()> {
    let args = Args::parse();
    commands::dispatch(args)
}