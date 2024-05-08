use serde::{Deserialize, Serialize};

use super::credentials::Credentials;

#[derive(Clone, Serialize, Deserialize)]
pub struct Vault {
    pub password_hash: String,
    pub credentials: Credentials,
}

impl Vault {
    pub fn new(password_hash: String, credentials: Credentials) -> Self {
        Vault {
            password_hash,
            credentials,
        }
    }
}
