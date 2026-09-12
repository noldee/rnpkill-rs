# rnpkill-rs

**A blazingly small, fast alternative to `npkill`, written in Rust.**

Interactively find and delete heavy dev folders — `node_modules`, virtualenvs, build artifacts — across any language ecosystem on your machine, right from the terminal.

> Rust reimplementation of [rnpkill](https://github.com/noldee/rnpkill) (the original Python version). Same idea, ~50x smaller binary, no runtime required.

---

## Why

Every dev machine ends up with gigabytes of `node_modules`, `venv`, `target`, and friends scattered across old projects nobody's touched in months. `rnpkill-rs` scans a directory tree, finds them all, shows you exactly how much space each one takes, and lets you delete the ones you don't need — interactively, safely, and fast.

## Features

- 🔍 **Cross-ecosystem detection** — Node.js, Python, Rust, Java, Go, PHP, Swift, Dart, Elixir, Terraform, and more out of the box.
- ⚡ **Fast, parallel size measurement** — powered by `rayon`, scans and measures large trees without blocking.
- 🎯 **Smart pruning** — never descends into a detected target folder, so a huge `node_modules` doesn't slow down the scan.
- 🖥️ **Interactive TUI** — arrow keys to navigate, space to mark, one keystroke to delete, live progress per folder.
- 🎨 **Themeable** — built-in themes: `default`, `dracula`, `mono`, `gruvbox`, `one-dark-pro`, `ayu`.
- 🕒 **Age filtering** — only show folders untouched for `90d`, `6m`, `1y`, etc.
- 📄 **Reports** — export a session's results to JSON or CSV.
- 🧪 **Dry-run mode** — see what would be deleted without touching anything.
- 🛡️ **Safe by default** — ambiguous folder names like `dist`, `build`, `bin`, `obj` are opt-in only (`--include-generic`), so it won't touch things that aren't build artifacts by default.

## Install

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/noldee/rnpkill-rs/main/install.sh | sh
```

### Windows (PowerShell)

```powershell
iwr https://raw.githubusercontent.com/noldee/rnpkill-rs/main/install.ps1 -useb | iex
```

Both scripts download the latest prebuilt binary from [Releases](https://github.com/noldee/rnpkill-rs/releases) and add it to your `PATH`.

### From source

```bash
git clone https://github.com/noldee/rnpkill-rs
cd rnpkill-rs
cargo build --release
# binary at target/release/rnpkill-rs
```

## Usage

```bash
rnpkill-rs [PATH] [OPTIONS]
```

Scans the current directory by default.

| Flag | Description |
|---|---|
| `--max-depth <N>` | Limit recursion depth |
| `--no-size` | Skip size measurement for an instant listing |
| `--older-than <EXPR>` | Only show folders untouched for at least this long (`90d`, `6m`, `1y`) |
| `--dry-run` | Show what would be deleted, without deleting anything |
| `--report <PATH>` | Export results to `.json` or `.csv` |
| `--include-generic` | Also detect `dist`, `build`, `bin`, `obj`, `out` (off by default — see below) |
| `--theme <NAME>` | `default`, `dracula`, `mono`, `gruvbox`, `onedark`, `ayu` |

### Examples

```bash
# Scan your projects folder, only folders untouched in 90+ days
rnpkill-rs ~/projects --older-than 90d

# See what would be freed without deleting anything
rnpkill-rs . --dry-run --report cleanup.json

# Use the Dracula theme
rnpkill-rs --theme dracula
```

### TUI controls

| Key | Action |
|---|---|
| `↑` / `↓` or `j` / `k` | Move cursor |
| `space` | Mark / unmark a folder |
| `a` | Mark / unmark all |
| `enter` | Delete marked folders |
| `q` / `esc` | Quit |
| `Ctrl+C` | Cancel without deleting |

### About `--include-generic`

Names like `dist`, `build`, `bin`, and `obj` are common build-output folders in some ecosystems, but in others they hold hand-written code or files actively served in production. They're excluded from the default scan on purpose — pass `--include-generic` only when you're sure they're safe to consider in your workflow.

## Building for other platforms

Release binaries are built automatically via GitHub Actions for:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

See [`.github/workflows/release.yml`](.github/workflows/release.yml).

## License

MIT © [Walter](https://github.com/noldee)
