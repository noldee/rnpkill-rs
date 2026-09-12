//! Traduce eventos de teclado crudos a acciones de UI, desacoplando
//! el input de la lógica del selector.

use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Home,
    End,
    Toggle,
    ToggleAll,
    Delete,
    Quit,
    Cancel,
    Noop,
}

pub fn map_key(code: KeyCode, modifiers: KeyModifiers) -> Action {
    match (code, modifiers) {
        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => Action::Up,
        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => Action::Down,
        (KeyCode::Home, _) => Action::Home,
        (KeyCode::End, _) => Action::End,
        (KeyCode::Char(' '), _) => Action::Toggle,
        (KeyCode::Char('a'), _) => Action::ToggleAll,
        (KeyCode::Enter, _) => Action::Delete,
        (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => Action::Quit,
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Action::Cancel,
        _ => Action::Noop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_movement_keys() {
        assert_eq!(map_key(KeyCode::Up, KeyModifiers::NONE), Action::Up);
        assert_eq!(
            map_key(KeyCode::Char('j'), KeyModifiers::NONE),
            Action::Down
        );
    }

    #[test]
    fn maps_ctrl_c_to_cancel() {
        assert_eq!(
            map_key(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Cancel
        );
    }
}
