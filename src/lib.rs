//! rnpkill-rs — Educational Rust reimplementation of rnpkill.
//!
//! Find and delete node_modules, venvs, and other heavy dev folders.
//!
//! This crate is organized in layers:
//! - `cli`: argument parsing and command dispatch
//! - `core`: pure domain logic (scanner, size, deleter, etc.)
//! - `utils`: cross-cutting helpers (formatters, paths)

pub mod cli;
pub mod core;
pub mod tui;
pub mod utils;