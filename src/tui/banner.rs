//! ASCII banner for rnpkill-rs.

use std::io::Write;
use std::time::Duration;

use crossterm::{
    cursor::MoveToColumn,
    queue,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};

use anyhow::Result;

use crate::tui::themes::Theme;
use crate::utils::formatters::{format_bytes, format_duration};

pub const BANNER: &str = r"
██████╗ ███╗   ██╗██████╗ ██╗  ██╗██╗██╗     ██╗
██╔══██╗████╗  ██║██╔══██╗██║ ██╔╝██║██║     ██║
██████╔╝██╔██╗ ██║██████╔╝█████╔╝ ██║██║     ██║
██╔══██╗██║╚██╗██║██╔═══╝ ██╔═██╗ ██║██║     ██║
██║  ██║██║ ╚████║██║     ██║  ██╗██║███████╗███████╗
╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝
";

/// Renders the banner using the given theme's `banner` color.
pub fn render<W: Write>(stdout: &mut W, theme: &Theme) -> Result<()> {
    queue!(
        stdout,
        SetForegroundColor(theme.banner),
        SetAttribute(Attribute::Bold),
    )?;

    for line in BANNER.lines() {
        queue!(stdout, MoveToColumn(0), Print(line), Print("\n"))?;
    }

    queue!(stdout, ResetColor, SetAttribute(Attribute::Reset))?;
    stdout.flush()?;
    Ok(())
}

/// Renders the "size total / scan time" subtitle right below the banner.
pub fn render_summary<W: Write>(
    stdout: &mut W,
    total_bytes: u64,
    scan_time: Duration,
    theme: &Theme,
) -> Result<()> {
    queue!(
        stdout,
        MoveToColumn(0),
        SetForegroundColor(theme.help),
        Print(format!(
            "  size total: {}   ·   scan time: {}\n\n",
            format_bytes(total_bytes),
            format_duration(scan_time),
        )),
        ResetColor,
    )?;
    stdout.flush()?;
    Ok(())
}
