use std::io;

use crate::app;
use crate::{
    app::app::{App, CurrentScreen, CurrentlyEditingCredentialField},
    ui::ui,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub fn handle_specific_credential(app: &mut App, key: KeyCode) -> Option<io::Result<bool>> {
    match key {
        KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
            app.cycle_editing_credential();
        }
        KeyCode::Up | KeyCode::BackTab => {
            app.reverse_cycle_editing_credential();
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::WebsiteCredentialScreen;
            app.discard_unsaved_credentials();
        }
        KeyCode::Char(value) => {
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
        // TODO: Change this to a different keybinding
        KeyCode::PageDown => {
            app.save_credential();
            app.current_screen = CurrentScreen::MainCredentialScreen;
        }
        _ => {}
    }

    return None;
}
