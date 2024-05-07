use std::io;

use crate::app::app::{App, CurrentScreen, CurrentlyEditingCredentialField};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_main_credentials(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Enter => {
            app.current_screen = CurrentScreen::WebsiteCredentialScreen;
            app.load_emails();
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            if app.unsaved_changes {
                app.current_screen = CurrentScreen::Exiting;
            } else {
                return Some(Ok(false));
            }
        }
        KeyCode::Char('n') => {
            app.current_screen = CurrentScreen::SpecificCredentialScreen;
            app.currently_editing_credential_field = Some(CurrentlyEditingCredentialField::Website);
        }
        KeyCode::Up | KeyCode::BackTab => {
            if app.selected_website_index > 0 {
                app.selected_website_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Tab => {
            if app.websites.len() == 0 {
                return None;
            }

            if app.selected_website_index < app.websites.len() - 1 {
                app.selected_website_index += 1;
            }
        }
        _ => {}
    }

    return None;
}
