use crate::app::app::{App, CurrentScreen};

use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_master_password(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Enter => {
            let password_input = app.master_password_input.clone();

            match app.load_credentials(&password_input) {
                Ok(_) => {
                    app.current_screen = CurrentScreen::MainCredentialScreen;
                }
                Err(_) => {}
            }

            app.master_password_input.clear();
        }
        KeyCode::Esc => {
            return Some(Ok(false));
        }
        KeyCode::Backspace => {
            app.master_password_input.pop();
        }
        KeyCode::Char(value) => {
            app.master_password_input.push(value);
        }
        _ => {}
    }

    return None;
}
