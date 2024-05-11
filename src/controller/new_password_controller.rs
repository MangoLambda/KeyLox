use crate::app::app::{App, CurrentScreen};

use crossterm::event::{KeyCode, KeyEvent};
use std::io;

// TODO: handle state better
pub fn handle_new_password(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Enter => {
            app.password_hash = app.new_password_input.clone();
            app.new_password_input.clear();
            app.current_screen = CurrentScreen::MainCredentialScreen;
        }
        KeyCode::Esc => {
            return Some(Ok(false));
        }
        KeyCode::Backspace => {
            // TODO
            app.new_password_input.pop();
        }
        KeyCode::Char(value) => {
            // TODO
            app.new_password_input.push(value);
        }
        _ => {}
    }

    return None;
}
