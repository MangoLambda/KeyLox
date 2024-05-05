use std::io;

use crate::app::app::{App, CurrentScreen};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn handle_master_password(app: &mut App, key: KeyCode) -> Option<io::Result<bool>> {
    match key {
        KeyCode::Enter => {
            if app.validate_master_password(&app.master_password_input) {
                app.current_screen = CurrentScreen::MainCredentialScreen;
                app.load_credentials();
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
