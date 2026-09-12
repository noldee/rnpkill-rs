//! Detección heurística de si la terminal soporta glifos Unicode
//! (box-drawing, flechas) correctamente.

use std::env;

/// Windows Terminal setea `WT_SESSION`. La mayoría de terminales
/// Unix/PowerShell moderno setean `TERM` distinto de "dumb". El
/// conhost/cmd.exe clásico no setea ninguna de las dos.
pub fn supports_unicode_glyphs() -> bool {
    if env::var("WT_SESSION").is_ok() {
        return true;
    }
    if let Ok(term) = env::var("TERM") {
        return term != "dumb";
    }
    cfg!(not(windows))
}
