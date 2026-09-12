//! Entry point of rnpkill-rs.
//!
//! Usage:
//!     rnpkill-rs [PATH]
//!
//! This MVP scans, measures, and lists. TUI and CLI options come later.

use std::env;
use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};

use rnpkill_rs::core::scanner::Scanner;
use rnpkill_rs::core::size::measure_many;
use rnpkill_rs::utils::formatters::format_bytes;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err:#}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // ─── Parsear argumento (por ahora solo `path`) ───────────────
    let root: PathBuf = env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string())
        .into();

    let root = root
        .canonicalize()
        .with_context(|| format!("Invalid path: {}", root.display()))?;

    println!("🔍 Scanning: {}\n", root.display());

    // ─── Escanear ─────────────────────────────────────────────────
    let scanner = Scanner::new(Some(6));
    let projects = scanner
        .scan(&root)
        .context("Failed to scan directory")?;

    // ─── Recolectar targets ───────────────────────────────────────
    let mut targets: Vec<_> = projects
        .into_iter()
        .flat_map(|p| p.targets)
        .collect();

    if targets.is_empty() {
        println!("No target folders found.");
        return Ok(());
    }

    // ─── Medir en paralelo ────────────────────────────────────────
    let paths: Vec<PathBuf> = targets.iter().map(|t| t.path.clone()).collect();
    let sizes = measure_many(&paths);

    for (target, size) in targets.iter_mut().zip(sizes.iter()) {
        target.size_bytes = *size;
    }

    // ─── Ordenar por tamaño descendente ───────────────────────────
    targets.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    // ─── Mostrar ──────────────────────────────────────────────────
    let total: u64 = targets.iter().map(|t| t.size_bytes).sum();

    for t in &targets {
        println!("{:>10}   {}", format_bytes(t.size_bytes), t.path.display());
    }

    println!(
        "\nTotal: {} · {} carpeta(s)",
        format_bytes(total),
        targets.len()
    );

    Ok(())
}