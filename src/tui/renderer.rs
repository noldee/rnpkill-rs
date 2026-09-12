//! Dibujo puro del selector — sin manejo de input ni mutación de estado.

use std::io::Write;
use std::time::SystemTime;

use anyhow::Result;
use crossterm::{
    cursor::{MoveToColumn, MoveToPreviousLine},
    queue,
    style::{Attribute, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::utils::formatters::format_bytes;
use crate::{
    tui::selector::{RowStatus, Selector},
    utils::formatters::format_duration,
};
pub fn draw_frame<W: std::io::Write>(selector: &Selector, stdout: &mut W) -> Result<()> {
    write_summary(selector, stdout)?;
    write_header(selector, stdout)?;
    for i in 0..selector.targets.len() {
        write_row(selector, stdout, i)?;
    }
    write_help(selector, stdout)?;
    stdout.flush()?;
    Ok(())
}

pub fn redraw<W: std::io::Write>(selector: &Selector, stdout: &mut W) -> Result<()> {
    queue!(
        stdout,
        MoveToPreviousLine(selector.frame_lines() as u16),
        MoveToColumn(0)
    )?;
    draw_frame(selector, stdout)
}
fn write_summary<W: std::io::Write>(selector: &Selector, stdout: &mut W) -> Result<()> {
    let theme = selector.theme();
    let remaining = selector.total_bytes.saturating_sub(selector.total_freed);

    queue!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0))?;
    queue!(
        stdout,
        SetForegroundColor(theme.help),
        Print(format!(
            "  size total: {}   ·   scan time: {}\n",
            format_bytes(remaining),
            format_duration(selector.scan_time),
        )),
        ResetColor,
    )?;
    queue!(
        stdout,
        Clear(ClearType::CurrentLine),
        MoveToColumn(0),
        Print("\n")
    )?;
    Ok(())
}

fn write_header<W: Write>(selector: &Selector, stdout: &mut W) -> Result<()> {
    let theme = selector.theme();
    let marked_bytes: u64 = selector
        .targets
        .iter()
        .enumerate()
        .filter(|(i, _)| selector.status[*i] == RowStatus::Marked)
        .map(|(_, t)| t.size_bytes)
        .sum();
    let marked_count = selector
        .status
        .iter()
        .filter(|s| **s == RowStatus::Marked)
        .count();

    queue!(
        stdout,
        Clear(ClearType::CurrentLine),
        MoveToColumn(0),
        SetForegroundColor(theme.header),
        SetAttribute(Attribute::Bold),
        Print(format!(
            "  {} marked · {}   ·   freed: {} · {} folder(s)\n",
            marked_count,
            format_bytes(marked_bytes),
            format_bytes(selector.total_freed),
            selector.targets.len()
        )),
        ResetColor,
        SetAttribute(Attribute::Reset),
    )?;
    Ok(())
}

fn write_row<W: Write>(selector: &Selector, stdout: &mut W, i: usize) -> Result<()> {
    let theme = selector.theme();
    let t = &selector.targets[i];
    let is_cursor = i == selector.cursor;
    let status = selector.status[i];

    let pointer = if is_cursor {
        if crate::utils::term::supports_unicode_glyphs() {
            "❯"
        } else {
            ">"
        }
    } else {
        " "
    };
    let size = format_bytes(t.size_bytes);
    let age = match t.mtime {
        Some(mtime) => human_age(mtime),
        None => "?".to_string(),
    };

    let prefix = match status {
        RowStatus::Idle => String::new(),
        RowStatus::Marked => "[X]".to_string(),
        RowStatus::Deleting => "[DELETING...]".to_string(),
        RowStatus::Deleted => "[DELETED]".to_string(),
        RowStatus::Error => "[ERROR]".to_string(),
    };

    let line = if prefix.is_empty() {
        format!(" {pointer}  {size:>10}  {age:>6}  {}\n", t.path.display())
    } else {
        format!(
            " {pointer} {prefix} {size:>10}  {age:>6}  {}\n",
            t.path.display()
        )
    };

    queue!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0))?;

    match status {
        RowStatus::Deleted => queue!(
            stdout,
            SetForegroundColor(theme.deleted),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
            SetAttribute(Attribute::Reset),
        )?,
        RowStatus::Deleting => queue!(
            stdout,
            SetForegroundColor(theme.deleting),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
            SetAttribute(Attribute::Reset),
        )?,
        RowStatus::Error => queue!(
            stdout,
            SetForegroundColor(theme.error),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
            SetAttribute(Attribute::Reset),
        )?,
        _ if is_cursor => queue!(
            stdout,
            SetBackgroundColor(theme.cursor_bg),
            SetForegroundColor(theme.cursor_fg),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
            SetAttribute(Attribute::Reset),
        )?,
        RowStatus::Marked => queue!(
            stdout,
            SetForegroundColor(theme.marked),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
            SetAttribute(Attribute::Reset),
        )?,
        RowStatus::Idle => queue!(stdout, Print(line))?,
    }

    Ok(())
}

fn write_help<W: Write>(selector: &Selector, stdout: &mut W) -> Result<()> {
    let theme = selector.theme();
    queue!(
        stdout,
        Clear(ClearType::CurrentLine),
        MoveToColumn(0),
        SetForegroundColor(theme.help),
        Print("  ↑/↓ move · space mark · a all · enter delete · q quit\n"),
        ResetColor,
    )?;
    Ok(())
}

fn human_age(mtime: SystemTime) -> String {
    let Ok(elapsed) = SystemTime::now().duration_since(mtime) else {
        return "?".to_string();
    };
    let secs = elapsed.as_secs();
    if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else if secs < 2_592_000 {
        format!("{}d", secs / 86400)
    } else if secs < 31_536_000 {
        format!("{}mo", secs / 2_592_000)
    } else {
        format!("{}y", secs / 31_536_000)
    }
}
