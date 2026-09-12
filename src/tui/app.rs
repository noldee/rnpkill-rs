//! Orquesta el TUI completo: prepara la terminal, dibuja el banner
//! y arranca el selector.

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::tui::banner;
use crate::tui::selector::Selector;
use crate::{core::models::Target, tui::themes::Theme};

pub struct App {
    targets: Vec<Target>,
    scan_time: Duration,
    theme: Theme,
}

impl App {
    pub fn new(targets: Vec<Target>, scan_time: Duration, theme: Theme) -> Self {
        Self {
            targets,
            scan_time,
            theme,
        }
    }

    /// Corre el TUI interactivo y devuelve los targets que se borraron.
    pub fn run(self) -> Result<Vec<Target>> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            cursor::Hide,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )?;

        banner::render(&mut stdout, &self.theme)?;

        let selector = Selector::with_theme(&self.targets, self.theme, self.scan_time);
        let result = selector.run(&mut stdout);

        execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        result
    }
}
