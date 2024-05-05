use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub credentials: Vec<Credential>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credential {
    pub website: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub notes: String,
}

impl Credentials {
    pub fn new() -> Self {
        Credentials {
            credentials: Vec::new(),
        }
    }

    pub fn add_or_update_credential(&mut self, credential: Credential) {
        let website = credential.website.clone();
        let email = credential.email.clone();
        let mut found = false;
        for c in &mut self.credentials {
            if c.website == website && c.email == email {
                c.username = credential.username.clone();
                c.password = credential.password.clone();
                c.notes = credential.notes.clone();
                found = true;
                break;
            }
        }
        if !found {
            self.credentials.push(credential);
        }
    }

    pub fn remove_credential(&mut self, website: &str, email: &str) {
        self.credentials
            .retain(|c| c.website != website || c.email != email);
    }

    pub fn get_websites(&self) -> Vec<String> {
        self.credentials
            .iter()
            .map(|c| c.website.clone())
            .unique()
            .collect()
    }

    pub fn get_emails(&self, website: &str) -> Vec<String> {
        self.credentials
            .iter()
            .filter(|c| c.website == website)
            .map(|c| c.email.clone())
            .collect()
    }

    pub fn get_credential(&self, website: &str, email: &str) -> Option<Credential> {
        for c in &self.credentials {
            if c.website == website && c.email == email {
                return Some(c.clone());
            }
        }
        None
    }
}

impl Credential {
    pub fn new(
        website: Option<String>,
        email: Option<String>,
        username: Option<String>,
        password: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Credential {
            website: website.unwrap_or_else(|| String::new()),
            email: email.unwrap_or_else(|| String::new()),
            username: username.unwrap_or_else(|| String::new()),
            password: password.unwrap_or_else(|| String::new()),
            notes: notes.unwrap_or_else(|| String::new()),
        }
    }
}
