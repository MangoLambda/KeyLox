use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod controller;
mod models;
mod ui;
use crate::{
    app::app::{App, CurrentScreen, CurrentlyEditingCredentialField},
    app::credentials_storage,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.save_changes()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::MasterPasswordRequired => {
                    if let Some(res) =
                        controller::master_password_controller::handle_master_password(
                            app, key.code,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::MainCredentialScreen => {
                    if let Some(res) =
                        controller::main_credentials_controller::handle_main_credentials(
                            app, key.code,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::WebsiteCredentialScreen => {
                    if let Some(res) =
                        controller::website_credentials_controller::handle_website_credentials(
                            app, key.code,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::SpecificCredentialScreen => {
                    if let Some(res) =
                        controller::specific_credential_controller::handle_specific_credential(
                            app, key.code,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::Exiting => {
                    if let Some(res) = controller::exit_controller::handle_exit(app, key.code) {
                        return res;
                    }
                }
            }
        }
    }
}
