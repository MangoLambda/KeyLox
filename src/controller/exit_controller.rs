use std::io;

use crate::app::app::App;

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_exit(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Char('y') => {
            return Some(Ok(true));
        }
        KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
            return Some(Ok(false));
        }
        _ => {}
    }

    return None;
}
