//! Estado interactivo del selector y el bucle de eventos.
//! El dibujo vive en `renderer`, el mapeo de teclas en `keybindings`.

use std::io::Write;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};

use crate::core::deleter::delete_target;
use crate::core::models::Target;
use crate::tui::keybindings::{map_key, Action};
use crate::tui::renderer;
use crate::tui::themes::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowStatus {
    Idle,
    Marked,
    Deleting,
    Deleted,
    Error,
}

pub struct Selector<'a> {
    pub(crate) targets: &'a [Target],
    pub(crate) cursor: usize,
    pub(crate) marked: Vec<bool>,
    pub(crate) status: Vec<RowStatus>,
    pub(crate) total_freed: u64,
    frame_lines: usize,
    theme: Theme,
}

impl<'a> Selector<'a> {
    pub fn with_theme(targets: &'a [Target], theme: Theme) -> Self {
        let n = targets.len();
        Self {
            targets,
            cursor: 0,
            marked: vec![false; n],
            status: vec![RowStatus::Idle; n],
            total_freed: 0,
            frame_lines: n + 2,
            theme,
        }
    }
    pub fn new(targets: &'a [Target]) -> Self {
        let n = targets.len();

        Self {
            targets,
            cursor: 0,
            marked: vec![false; n],
            status: vec![RowStatus::Idle; n],
            total_freed: 0,
            // header (1) + rows (n) + help (1)
            frame_lines: n + 2,
            theme: Theme::default(),
        }
    }

    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    pub(crate) fn frame_lines(&self) -> usize {
        self.frame_lines
    }

    /// Corre el loop interactivo. Asume que quien llama ya preparó
    /// la terminal (raw mode, alt screen) e imprimió el banner —
    /// eso ahora vive en `App::run`.
    pub fn run<W: Write>(mut self, stdout: &mut W) -> Result<Vec<Target>> {
        renderer::draw_frame(&self, stdout)?;
        self.event_loop(stdout)
    }

    fn event_loop<W: Write>(&mut self, stdout: &mut W) -> Result<Vec<Target>> {
        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match map_key(code, modifiers) {
                    Action::Up => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    Action::Down => {
                        if self.cursor + 1 < self.targets.len() {
                            self.cursor += 1;
                        }
                    }
                    Action::Home => self.cursor = 0,
                    Action::End => self.cursor = self.targets.len().saturating_sub(1),
                    Action::Toggle => match self.status[self.cursor] {
                        RowStatus::Idle => {
                            self.marked[self.cursor] = true;
                            self.status[self.cursor] = RowStatus::Marked;
                        }
                        RowStatus::Marked => {
                            self.marked[self.cursor] = false;
                            self.status[self.cursor] = RowStatus::Idle;
                        }
                        _ => {}
                    },
                    Action::ToggleAll => self.toggle_all(),
                    Action::Delete => self.run_deletion(stdout)?,
                    Action::Quit => return Ok(self.collect_deleted()),
                    Action::Cancel => return Ok(Vec::new()),
                    Action::Noop => {}
                }
            }

            renderer::redraw(self, stdout)?;
        }
    }

    fn toggle_all(&mut self) {
        let all_marked = self
            .status
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == RowStatus::Idle || **s == RowStatus::Marked)
            .all(|(i, _)| self.marked[i]);

        for i in 0..self.targets.len() {
            if self.status[i] == RowStatus::Idle && !all_marked {
                self.marked[i] = true;
                self.status[i] = RowStatus::Marked;
            } else if self.status[i] == RowStatus::Marked && all_marked {
                self.marked[i] = false;
                self.status[i] = RowStatus::Idle;
            }
        }
    }

    fn collect_deleted(&self) -> Vec<Target> {
        self.targets
            .iter()
            .zip(self.status.iter())
            .filter(|(_, s)| **s == RowStatus::Deleted)
            .map(|(t, _)| t.clone())
            .collect()
    }

    fn run_deletion<W: Write>(&mut self, stdout: &mut W) -> Result<()> {
        let to_delete: Vec<usize> = self
            .status
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == RowStatus::Marked)
            .map(|(i, _)| i)
            .collect();

        if to_delete.is_empty() {
            return Ok(());
        }

        for &i in &to_delete {
            self.status[i] = RowStatus::Deleting;
            renderer::redraw(self, stdout)?;
            std::thread::sleep(std::time::Duration::from_millis(200));

            match delete_target(&self.targets[i]) {
                Ok(bytes) => {
                    self.total_freed += bytes;
                    self.marked[i] = false;
                    self.status[i] = RowStatus::Deleted;
                }
                Err(_) => self.status[i] = RowStatus::Error,
            }

            renderer::redraw(self, stdout)?;
        }

        Ok(())
    }
}
