//! Color theme for the TUI, separado del renderer para no tener
//! colores hardcodeados en la lógica de dibujo.

use crossterm::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub banner: Color,
    pub header: Color,
    pub idle: Color,
    pub marked: Color,
    pub deleting: Color,
    pub deleted: Color,
    pub error: Color,
    pub cursor_bg: Color,
    pub cursor_fg: Color,
    pub help: Color,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            banner: Color::Cyan,
            header: Color::Cyan,
            idle: Color::Reset,
            marked: Color::Cyan,
            deleting: Color::Yellow,
            deleted: Color::Green,
            error: Color::Red,
            cursor_bg: Color::Magenta,
            cursor_fg: Color::White,
            help: Color::DarkGrey,
        }
    }
}

impl Theme {
    pub fn dracula() -> Self {
        Self {
            banner: rgb(189, 147, 249),
            header: rgb(189, 147, 249), // purple
            idle: Color::Reset,
            marked: rgb(139, 233, 253),   // cyan
            deleting: rgb(241, 250, 140), // yellow
            deleted: rgb(80, 250, 123),   // green
            error: rgb(255, 85, 85),      // red
            cursor_bg: rgb(68, 71, 90),
            cursor_fg: rgb(248, 248, 242),
            help: Color::DarkGrey,
        }
    }

    pub fn mono() -> Self {
        Self {
            banner: Color::White,
            header: Color::White,
            idle: Color::Reset,
            marked: Color::White,
            deleting: Color::Grey,
            deleted: Color::Grey,
            error: Color::White,
            cursor_bg: Color::White,
            cursor_fg: Color::Black,
            help: Color::DarkGrey,
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            banner: rgb(250, 189, 47), // bright yellow
            header: rgb(184, 187, 38), // bright green
            idle: Color::Reset,
            marked: rgb(131, 165, 152),    // aqua
            deleting: rgb(250, 189, 47),   // yellow
            deleted: rgb(184, 187, 38),    // green
            error: rgb(251, 73, 52),       // bright red
            cursor_bg: rgb(80, 73, 69),    // bg2
            cursor_fg: rgb(235, 219, 178), // fg1
            help: rgb(146, 131, 116),      // gray
        }
    }

    pub fn one_dark_pro() -> Self {
        Self {
            banner: rgb(198, 120, 221), // purple
            header: rgb(97, 175, 239),  // blue
            idle: Color::Reset,
            marked: rgb(86, 182, 194),     // cyan
            deleting: rgb(229, 192, 123),  // yellow
            deleted: rgb(152, 195, 121),   // green
            error: rgb(224, 108, 117),     // red
            cursor_bg: rgb(62, 68, 81),    // selection bg
            cursor_fg: rgb(171, 178, 191), // fg
            help: rgb(92, 99, 112),        // comment grey
        }
    }

    pub fn ayu() -> Self {
        Self {
            banner: rgb(255, 180, 84), // orange accent
            header: rgb(89, 194, 255), // blue
            idle: Color::Reset,
            marked: rgb(149, 230, 203),  // teal/green
            deleting: rgb(255, 180, 84), // orange
            deleted: rgb(170, 217, 76),  // green
            error: rgb(255, 51, 51),     // red
            cursor_bg: rgb(48, 55, 68),
            cursor_fg: rgb(230, 225, 207),
            help: rgb(92, 103, 115),
        }
    }

    /// Resuelve el nombre pasado por `--theme` a un `Theme`.
    /// Nombre desconocido -> cae al default sin romper nada.
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dracula" => Self::dracula(),
            "mono" | "monochrome" => Self::mono(),
            "gruvbox" => Self::gruvbox(),
            "onedark" | "one-dark" | "one-dark-pro" => Self::one_dark_pro(),
            "ayu" => Self::ayu(),
            _ => Self::default(),
        }
    }
}
