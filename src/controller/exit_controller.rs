use std::io;

use crate::app::app::App;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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
