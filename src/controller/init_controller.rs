use crate::app::{
    app::{App, CurrentScreen},
    credentials_storage::are_credentials_present,
};

pub fn handle_init(app: &mut App) {
    if !are_credentials_present() {
        app.current_screen = CurrentScreen::NewPasswordRequiredScreen;
    } else {
        app.current_screen = CurrentScreen::MasterPasswordRequiredScreen;
    }
}
