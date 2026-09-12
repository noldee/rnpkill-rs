//! Byte formatting helpers.

use std::time::Duration;

const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];

/// Formats a byte count into a human-readable string like `1.24 GB`.
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.2} {}", size, UNITS[unit])
    }
}

/// Formatea una duración en algo legible: `340ms`, `1.2s`, `2m 05s`.
pub fn format_duration(d: Duration) -> String {
    let millis = d.as_millis();
    if millis < 1000 {
        format!("{millis}ms")
    } else {
        let secs = d.as_secs();
        if secs < 60 {
            format!("{:.1}s", d.as_secs_f64())
        } else {
            format!("{}m {:02}s", secs / 60, secs % 60)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_zero() {
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn formats_bytes() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn formats_kilobytes() {
        assert_eq!(format_bytes(2048), "2.00 KB");
    }

    #[test]
    fn formats_megabytes() {
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    }

    #[test]
    fn formats_gigabytes() {
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 3 / 2), "1.50 GB");
    }
}
