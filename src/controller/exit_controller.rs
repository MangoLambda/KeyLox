use crate::app::app::{App, CurrentScreen};

use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_exit(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Char('y') => {
            return Some(Ok(true));
        }
        KeyCode::Char('n') => {
            return Some(Ok(false));
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::MainCredentialScreen;
        }
        _ => {}
    }

    return None;
}
