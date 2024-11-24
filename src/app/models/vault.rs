use serde::{Deserialize, Serialize};

use super::vault_security_params::VaultSecurityParams;

#[derive(Clone, Serialize, Deserialize)]
pub struct Vault {
    pub vault_security_params: VaultSecurityParams,
    pub protected_data: Vec<u8>,
}

impl Vault {
    pub fn new(vault_security_params: VaultSecurityParams, protected_data: &[u8]) -> Self {
        Vault {
            vault_security_params: vault_security_params,
            protected_data: protected_data.to_vec(),
        }
    }
}
