use crate::app::app::{App, CurrentScreen};

use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_master_password(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Enter => {
            let password_input = app.master_password_input.clone();

            if app.load_credentials(&password_input).is_ok() {
                app.current_screen = CurrentScreen::MainCredentialScreen;
            }

            //app.master_key = app.master_password_input.clone();
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
