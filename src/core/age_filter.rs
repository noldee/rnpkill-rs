//! Age filter for `--older-than`.

use std::time::{Duration, SystemTime};

use anyhow::{anyhow, Result};

use crate::core::models::Target;

/// Parses an age expression like `90d`, `6m`, `1y` into a `Duration`.
///
/// Supported units: `h` (hours), `d` (days), `w` (weeks),
/// `m` (30-day months), `y` (365-day years).
pub fn parse_age(expr: &str) -> Result<Duration> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err(anyhow!("empty age expression"));
    }

    let (num_str, unit) = if expr.chars().last().unwrap().is_ascii_digit() {
        (expr, "d") // default unit is days
    } else {
        let (n, u) = expr.split_at(expr.len() - 1);
        (n, u)
    };

    let value: u64 = num_str
        .parse()
        .map_err(|_| anyhow!("invalid number in age expression: '{}'", expr))?;

    let seconds = match unit.to_lowercase().as_str() {
        "h" => value * 3600,
        "d" => value * 86400,
        "w" => value * 604800,
        "m" => value * 2_592_000,  // 30 days
        "y" => value * 31_536_000, // 365 days
        other => return Err(anyhow!("unknown time unit: '{}'", other)),
    };

    Ok(Duration::from_secs(seconds))
}

pub struct AgeFilter {
    threshold: Option<Duration>,
}

impl AgeFilter {
    pub fn from_expression(expr: Option<&str>) -> Result<Self> {
        let threshold = match expr {
            Some(e) => Some(parse_age(e)?),
            None => None,
        };
        Ok(Self { threshold })
    }

    pub fn is_active(&self) -> bool {
        self.threshold.is_some()
    }

    /// Returns only targets older than the threshold.
    pub fn apply(&self, targets: Vec<Target>) -> Vec<Target> {
        let Some(threshold) = self.threshold else {
            return targets;
        };

        let now = SystemTime::now();
        let cutoff = now.checked_sub(threshold).unwrap_or(SystemTime::UNIX_EPOCH);

        targets
            .into_iter()
            .filter(|t| match t.mtime {
                Some(mtime) => mtime < cutoff,
                None => false,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_days() {
        assert_eq!(parse_age("90d").unwrap().as_secs(), 90 * 86400);
    }

    #[test]
    fn parses_default_unit() {
        assert_eq!(parse_age("90").unwrap().as_secs(), 90 * 86400);
    }

    #[test]
    fn parses_months() {
        assert_eq!(parse_age("6m").unwrap().as_secs(), 6 * 2_592_000);
    }

    #[test]
    fn parses_years() {
        assert_eq!(parse_age("1y").unwrap().as_secs(), 31_536_000);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_age("abc").is_err());
        assert!(parse_age("90x").is_err());
    }
}