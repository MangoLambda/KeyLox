use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Terminal};
use std::{error::Error, io};

mod app;
mod controller;
mod errors;
mod models;
mod tui;
mod ui;
use crate::{
    app::app::{App, CurrentScreen},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup panic hook
    errors::install_hooks()?;

    // setup terminal
    let mut terminal = tui::init()?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    tui::restore()?;

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
    controller::init_controller::handle_init(app);

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Init => {
                    // todo remove this?
                }
                CurrentScreen::NewPasswordRequiredScreen => {
                    if let Some(res) =
                        controller::new_password_controller::handle_new_password(app, key_event)
                    {
                        return res;
                    }
                }
                CurrentScreen::MasterPasswordRequiredScreen => {
                    if let Some(res) =
                        controller::master_password_controller::handle_master_password(
                            app, key_event,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::MainCredentialScreen => {
                    if let Some(res) =
                        controller::main_credentials_controller::handle_main_credentials(
                            app, key_event,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::WebsiteCredentialScreen => {
                    if let Some(res) =
                        controller::website_credentials_controller::handle_website_credentials(
                            app, key_event,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::SpecificCredentialScreen => {
                    if let Some(res) =
                        controller::specific_credential_controller::handle_specific_credential(
                            app, key_event,
                        )
                    {
                        return res;
                    }
                }
                CurrentScreen::Exiting => {
                    if let Some(res) = controller::exit_controller::handle_exit(app, key_event) {
                        return res;
                    }
                }
            }
        }
    }
}
