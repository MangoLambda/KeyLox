use std::io;

use crate::app::app::{App, CurrentScreen, CurrentlyEditingCredentialField};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_website_credentials(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        KeyCode::Enter => {
            app.current_screen = CurrentScreen::SpecificCredentialScreen;
            app.currently_editing_credential_field =
                Some(CurrentlyEditingCredentialField::Username);
            app.load_credential();
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_screen = CurrentScreen::MainCredentialScreen;
            app.discard_unsaved_credentials();
        }
        KeyCode::Char('n') => {
            app.current_screen = CurrentScreen::SpecificCredentialScreen;
            app.currently_editing_credential_field = Some(CurrentlyEditingCredentialField::Email);

            if app.credentials.get_websites().len() > app.selected_website_index {
                app.website_input =
                    app.credentials.get_websites()[app.selected_website_index].clone();
            }
        }
        KeyCode::Up | KeyCode::BackTab => {
            if app.selected_email_index > 0 {
                app.selected_email_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Tab => {
            if app.emails.len() == 0 {
                return None;
            }
            if app.selected_email_index < app.emails.len() - 1 {
                app.selected_email_index += 1;
            }
        }
        KeyCode::Backspace => {
            app.remove_selected_credential();
        }
        _ => {}
    }

    return None;
}
