use crate::app::app::{App, CurrentScreen, CurrentlyEditingCredentialField};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;

pub fn handle_specific_credential(app: &mut App, key_event: KeyEvent) -> Option<io::Result<bool>> {
    match key_event.code {
        //TODO: Refactor to use key_events
        KeyCode::Up | KeyCode::BackTab => {
            app.reverse_cycle_editing_credential();
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::WebsiteCredentialScreen;
            app.discard_unsaved_credentials();
        }
        KeyCode::Backspace => {
            if let Some(editing) = &app.currently_editing_credential_field {
                match editing {
                    CurrentlyEditingCredentialField::Website => {
                        app.website_input.pop();
                    }
                    CurrentlyEditingCredentialField::Email => {
                        app.email_input.pop();
                    }
                    CurrentlyEditingCredentialField::Username => {
                        app.username_input.pop();
                    }
                    CurrentlyEditingCredentialField::Password => {
                        app.password_input.pop();
                    }
                    CurrentlyEditingCredentialField::Notes => {
                        app.notes_input.pop();
                    }
                }
            }
        }
        _ => {}
    }

    match key_event {
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            kind: _,
            state: _,
        } => {
            app.save_credential();
            app.current_screen = CurrentScreen::MainCredentialScreen;
        }
        KeyEvent {
            code: KeyCode::Enter | KeyCode::Tab | KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: _,
            state: _,
        } => {
            app.cycle_editing_credential();
        }
        KeyEvent {
            code: KeyCode::Char(value),
            modifiers: KeyModifiers::NONE,
            kind: _,
            state: _,
        } => {
            if let Some(editing) = &app.currently_editing_credential_field {
                match editing {
                    CurrentlyEditingCredentialField::Website => {
                        app.website_input.push(value);
                    }
                    CurrentlyEditingCredentialField::Email => {
                        app.email_input.push(value);
                    }
                    CurrentlyEditingCredentialField::Username => {
                        app.username_input.push(value);
                    }
                    CurrentlyEditingCredentialField::Password => {
                        app.password_input.push(value);
                    }
                    CurrentlyEditingCredentialField::Notes => {
                        app.notes_input.push(value);
                    }
                }
            }
        }
        _ => {}
    }

    return None;
}
