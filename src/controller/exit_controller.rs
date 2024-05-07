use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_exit(key_event: KeyEvent) -> Option<io::Result<bool>> {
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
