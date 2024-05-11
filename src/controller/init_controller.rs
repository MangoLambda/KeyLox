use crate::app::app::{App, CurrentScreen};

pub fn handle_init(app: &mut App) {
    app.load_credentials();

    if app.password_hash.is_empty() {
        app.current_screen = CurrentScreen::NewPasswordRequiredScreen;
    } else {
        app.current_screen = CurrentScreen::MasterPasswordRequiredScreen;
    }
}
