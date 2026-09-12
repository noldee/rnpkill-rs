//! JSON/CSV report export.

use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::core::models::Target;

#[derive(Serialize)]
struct TargetRow {
    name: String,
    path: String,
    parent: String,
    size_bytes: u64,
    mtime: Option<String>,
}

#[derive(Serialize)]
struct ReportPayload {
    generated_at: String,
    total_count: usize,
    total_bytes: u64,
    targets: Vec<TargetRow>,
}

/// Writes a report to `path`. Format inferred from extension (`.json` or `.csv`).
pub fn write_report(targets: &[Target], path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("json")
        .to_lowercase();

    match ext.as_str() {
        "csv" => write_csv(targets, path),
        _ => write_json(targets, path),
    }
}

fn write_json(targets: &[Target], path: &Path) -> Result<()> {
    let payload = ReportPayload {
        generated_at: chrono::Local::now().to_rfc3339(),
        total_count: targets.len(),
        total_bytes: targets.iter().map(|t| t.size_bytes).sum(),
        targets: targets
            .iter()
            .map(|t| TargetRow {
                name: t.name.clone(),
                path: t.path.display().to_string(),
                parent: t
                    .path
                    .parent()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
                size_bytes: t.size_bytes,
                mtime: t.mtime.map(|m| {
                    chrono::DateTime::<chrono::Local>::from(m).to_rfc3339()
                }),
            })
            .collect(),
    };

    let json = serde_json::to_string_pretty(&payload)
        .context("Failed to serialize JSON")?;

    std::fs::write(path, json)
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn write_csv(targets: &[Target], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .with_context(|| format!("Failed to create {}", path.display()))?;

    wtr.write_record(["name", "path", "parent", "size_bytes", "mtime"])?;

    for t in targets {
        wtr.write_record(&[
            t.name.clone(),
            t.path.display().to_string(),
            t.path
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
            t.size_bytes.to_string(),
            t.mtime
                .map(|m| chrono::DateTime::<chrono::Local>::from(m).to_rfc3339())
                .unwrap_or_default(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}