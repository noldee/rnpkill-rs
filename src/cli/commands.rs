//! Command dispatch and main flow.

use crate::tui::themes::Theme;
use anyhow::{Context, Result};
use std::time::Instant;

use crate::cli::args::{Args, Command};
use crate::core::age_filter::AgeFilter;
use crate::core::models::Target;
use crate::core::reporter::write_report;
use crate::core::scanner::Scanner;
use crate::core::size::measure_many;
use crate::utils::formatters::format_bytes;

/// Entry point called from `main.rs`.
pub fn dispatch(args: Args) -> Result<()> {
    // ─── Subcommands (history / stats / export-history) ────────
    if let Some(cmd) = &args.command {
        return match cmd {
            Command::History => {
                println!("TODO: history command (Phase 4)");
                Ok(())
            }
            Command::Stats => {
                println!("TODO: stats command (Phase 4)");
                Ok(())
            }
            Command::ExportHistory { path } => {
                println!("TODO: export history to {} (Phase 4)", path.display());
                Ok(())
            }
            Command::Upgrade => crate::cli::upgrade::run(),
        };
    }

    run_scan(args)
}

fn run_scan(args: Args) -> Result<()> {
    let root = args
        .path
        .canonicalize()
        .with_context(|| format!("Invalid path: {}", args.path.display()))?;

    println!("🔍 Scanning: {}\n", root.display());

    let scan_started = Instant::now();

    let scanner = Scanner::with_options(args.max_depth, args.include_generic);
    let projects = scanner.scan(&root).context("Failed to scan directory")?;

    let mut targets: Vec<_> = projects.into_iter().flat_map(|p| p.targets).collect();

    if targets.is_empty() {
        println!("No target folders found.");
        return Ok(());
    }

    if let Some(expr) = &args.older_than {
        let filter = AgeFilter::from_expression(Some(expr))?;
        let before = targets.len();
        targets = filter.apply(targets);
        println!(
            "Filtered by age: {}/{} folder(s) match.\n",
            targets.len(),
            before
        );
    }

    if targets.is_empty() {
        println!("No targets left after filtering.");
        return Ok(());
    }

    if !args.no_size {
        let paths: Vec<_> = targets.iter().map(|t| t.path.clone()).collect();
        let sizes = measure_many(&paths);
        for (t, size) in targets.iter_mut().zip(sizes.iter()) {
            t.size_bytes = *size;
        }
    }

    let scan_time = scan_started.elapsed();

    targets.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    let total: u64 = targets.iter().map(|t| t.size_bytes).sum();

    // ─── Salida en consola ──────────────────────────────────────
    // El listado completo solo se imprime en dry-run, donde es la
    // única salida. En modo interactivo, la TUI ya lo muestra —
    // duplicarlo en la consola plana es ruido.
    if args.dry_run {
        for (i, t) in targets.iter().enumerate() {
            println!(
                "  [{:>2}] {:>10}   {}",
                i,
                format_bytes(t.size_bytes),
                t.path.display()
            );
        }
        println!(
            "\nTotal: {} · {} folder(s)",
            format_bytes(total),
            targets.len()
        );
    } else {
        println!(
            "Found {} folder(s) · {}",
            targets.len(),
            format_bytes(total)
        );
    }

    if let Some(report_path) = &args.report {
        write_report(&targets, report_path)?;
        println!("\n📄 Report saved: {}", report_path.display());
    }

    if args.dry_run {
        println!("\n[DRY RUN] No folders were deleted.");
        return Ok(());
    }

    interactive_delete(targets, scan_time, &args.theme)
}

fn interactive_delete(
    targets: Vec<Target>,
    scan_time: std::time::Duration,
    theme_name: &str,
) -> Result<()> {
    use crate::tui::app::App;

    println!("\nSelect folders to delete:\n");

    let deleted = App::new(targets, scan_time, Theme::from_name(theme_name)).run()?;

    if deleted.is_empty() {
        println!("Nothing deleted.");
        return Ok(());
    }

    let total_bytes: u64 = deleted.iter().map(|t| t.size_bytes).sum();
    println!("\n✓ Freed: {}", format_bytes(total_bytes));
    println!("  {} folder(s) removed", deleted.len());

    Ok(())
}
