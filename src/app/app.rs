use crate::models::{
    credentials::{Credential, Credentials},
    vault::Vault,
};

use super::credentials_storage::{self, load_credentials};
use std::error::Error;

pub enum CurrentScreen {
    Init,
    NewPasswordRequiredScreen,
    MasterPasswordRequiredScreen,
    MainCredentialScreen,
    WebsiteCredentialScreen,
    SpecificCredentialScreen,
    Exiting,
}

pub enum CurrentlyEditingCredentialField {
    Website,
    Email,
    Username,
    Password,
    Notes,
}

pub struct App {
    pub credentials_file_exists: bool,

    pub unsaved_changes: bool, // a flag to determine if there are unsaved changes.
    pub websites: Vec<String>, // the list of credentials that the user has saved.
    pub selected_website_index: usize, // the currently selected credential.
    pub emails: Vec<String>,   // the list of emails that the user has saved.
    pub selected_email_index: usize, // the currently selected email.
    pub currently_editing_credential_field: Option<CurrentlyEditingCredentialField>, // the optional state containing which of the username or password the user is editing. It is an option, because when the user is not directly editing a credential, this will be set to `None`.

    pub password_hash: String,
    pub credentials: Credentials,

    pub new_password_input: String, // the new password that the user is trying to set.
    pub master_password_input: String, // the currently being edited master password.

    pub website_input: String,
    pub email_input: String,
    pub username_input: String,
    pub password_input: String,
    pub notes_input: String,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditingCredentialField>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        let app = App {
            credentials_file_exists: false,
            unsaved_changes: true,
            websites: Vec::new(),
            selected_website_index: 0,
            emails: Vec::new(),
            selected_email_index: 0,
            currently_editing_credential_field: None,

            new_password_input: String::new(),
            master_password_input: String::new(),

            website_input: String::new(),
            email_input: String::new(),
            username_input: String::new(),
            password_input: String::new(),
            notes_input: String::new(),
            current_screen: CurrentScreen::Init,
            currently_editing: None,

            credentials: Credentials::new(),
            password_hash: String::new(),
        };

        load_credentials().unwrap();

        return app;
    }

    pub fn load_credentials(&mut self) {
        // TODO: error handling
        if let Some(vault) = credentials_storage::load_credentials().unwrap() {
            self.password_hash = vault.password_hash.clone();
            self.credentials = vault.credentials.clone();
            self.credentials_file_exists = true;
        }

        self.websites = self.credentials.get_websites();
    }

    pub fn load_emails(&mut self) {
        // TODO: refactor
        if self.websites.len() == 0 {
            self.emails = Vec::new();
            return;
        }

        self.selected_website_index =
            std::cmp::min(self.selected_website_index, self.websites.len() - 1);

        if self.selected_website_index >= self.websites.len() {
            // TODO: log
            return;
        }

        let website = &self.websites[self.selected_website_index];
        self.emails = self.credentials.get_emails(&website);
    }

    pub fn load_credential(&mut self) {
        if self.selected_website_index >= self.websites.len() {
            self.selected_website_index = 0;
            // TODO: log
            return;
        }
        if self.selected_email_index >= self.emails.len() {
            self.selected_email_index = 0;
            // todo: log
            return;
        }
        let website = &self.websites[self.selected_website_index];
        let email = &self.emails[self.selected_email_index];

        if let Some(credential) = self.credentials.get_credential(website, email) {
            self.website_input = credential.website.clone();
            self.email_input = credential.email.clone();
            self.username_input = credential.username.clone();
            self.password_input = credential.password.clone();
            self.notes_input = credential.notes.clone();
        }
    }

    pub fn discard_unsaved_credentials(&mut self) {
        self.website_input.clear();
        self.email_input.clear();
        self.username_input.clear();
        self.password_input.clear();
        self.notes_input.clear();

        self.website_input.clear();
        self.email_input.clear();
        self.username_input.clear();
        self.password_input.clear();
        self.notes_input.clear();
        self.currently_editing = None;
    }

    pub fn remove_selected_credential(&mut self) {
        if self.websites.len() == 0 {
            return;
        }
        if self.emails.len() == 0 {
            return;
        }

        // TODO: make this better
        if self.websites.len() <= self.selected_website_index
            || self.emails.len() <= self.selected_email_index
        {
            self.selected_website_index = self.websites.len() - 1;
            self.selected_email_index = self.emails.len() - 1;
            return;
        }

        let website = &self.websites[self.selected_website_index];
        let email = &self.emails[self.selected_email_index];

        self.credentials.remove_credential(website, email);
        self.websites = self.credentials.get_websites();
        self.load_emails();
        self.discard_unsaved_credentials();
    }

    pub fn save_credential(&mut self) {
        let credential = Credential::new(
            Some(self.website_input.clone()),
            Some(self.email_input.clone()),
            Some(self.username_input.clone()),
            Some(self.password_input.clone()),
            Some(self.notes_input.clone()),
        );

        self.credentials.add_or_update_credential(credential);
        self.websites = self.credentials.get_websites();

        self.discard_unsaved_credentials();
    }

    pub fn cycle_editing_credential(&mut self) {
        if let Some(edit_mode) = &self.currently_editing_credential_field {
            match edit_mode {
                CurrentlyEditingCredentialField::Website => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Email)
                }
                CurrentlyEditingCredentialField::Email => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Username)
                }
                CurrentlyEditingCredentialField::Username => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Password)
                }
                CurrentlyEditingCredentialField::Password => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Notes)
                }
                CurrentlyEditingCredentialField::Notes => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Website)
                }
            };
        } else {
            self.currently_editing = Some(CurrentlyEditingCredentialField::Website);
        }
    }

    pub fn reverse_cycle_editing_credential(&mut self) {
        if let Some(edit_mode) = &self.currently_editing_credential_field {
            match edit_mode {
                CurrentlyEditingCredentialField::Website => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Notes)
                }
                CurrentlyEditingCredentialField::Email => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Website)
                }
                CurrentlyEditingCredentialField::Username => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Email)
                }
                CurrentlyEditingCredentialField::Password => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Username)
                }
                CurrentlyEditingCredentialField::Notes => {
                    self.currently_editing_credential_field =
                        Some(CurrentlyEditingCredentialField::Password)
                }
            };
        } else {
            self.currently_editing = Some(CurrentlyEditingCredentialField::Website);
        }
    }

    pub fn save_changes(&self) -> Result<(), Box<dyn Error>> {
        // TODO: error handling
        let vault = Vault::new(self.password_hash.clone(), self.credentials.clone());
        credentials_storage::store_vault(&vault)?;
        println!("Changes saved");
        Ok(())
    }

    pub fn validate_master_password(&mut self, password: &String) -> bool {
        return password == &self.password_hash;
    }
}
