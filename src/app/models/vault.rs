use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Vault {
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub protected_data: Vec<u8>,
    pub auth_tag: Vec<u8>,
}

impl Vault {
    pub fn new(salt: &[u8], nonce: &[u8], protected_data: &[u8], auth_tag: &[u8]) -> Self {
        Vault {
            salt: salt.to_vec(),
            nonce: nonce.to_vec(),
            protected_data: protected_data.to_vec(),
            auth_tag: auth_tag.to_vec(),
        }
    }
}
