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
pub const BANNER_PLAIN: &str = r"
 ____  _   _ ____  _  ___ _     _
|  _ \| \ | |  _ \| |/ / |_| |   | |
| |_) |  \| | |_) | ' /   | |   | |
|  _ <| |\  |  __/| . \   | |___| |___
|_| \_\_| \_|_|   |_|\_\  |_____|_____|
";

/// Renders the banner using the given theme's `banner` color.
pub fn render<W: Write>(stdout: &mut W, theme: &Theme) -> Result<()> {
    let art = if crate::utils::term::supports_unicode_glyphs() {
        BANNER
    } else {
        BANNER_PLAIN
    };

    queue!(
        stdout,
        SetForegroundColor(theme.banner),
        SetAttribute(Attribute::Bold)
    )?;
    for line in art.lines() {
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
