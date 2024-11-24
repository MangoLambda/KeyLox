use crate::app::encryption;
use crate::app::models::credentials::Credentials;
use crate::app::models::vault::Vault;

use super::models::vault_security_params::{AegisParams, EncryptionParamsEnum};

// TODO: Change salt to object containing info to generate the key material
pub fn encrypt(key: &Vec<u8>, credentials: Credentials) -> (EncryptionParamsEnum, Vec<u8>) {
    // TODO:Error handling
    //let mut serialized_credentials = serde_json::to_string(&credentials).unwrap().into_bytes();
    let mut serialized_credentials = rmp_serde::to_vec(&credentials).unwrap();
    let (nonce, auth_tag) = encryption::encrypt_in_place(key, &[], &mut serialized_credentials);

    let encryption_params =
        EncryptionParamsEnum::Aegis(AegisParams::new(32, nonce.to_vec(), auth_tag.to_vec()));
    (encryption_params, serialized_credentials)
}

pub fn decrypt(key: &Vec<u8>, vault: Vault) -> Result<Credentials, encryption::DecryptionError> {
    let mut serialized_credentials = vault.protected_data.clone();

    // TODO: Make error for encryption::DecryptionError::UnsupportedEncryptionParams
    let (nonce, auth_tag) = match vault.vault_security_params.get_encryption_params() {
        EncryptionParamsEnum::Aegis(aegis_params) => {
            let nonce = aegis_params.get_nonce();
            let auth_tag = aegis_params.get_auth_tag();
            (nonce, auth_tag)
        }
    };

    encryption::decrypt_in_place(
        key.as_slice(),
        nonce.as_slice(),
        auth_tag.as_slice(),
        &[],
        &mut serialized_credentials,
    )?;

    // TODO: Error handling
    //let credentials: Credentials = serde_json::from_slice(&serialized_credentials).unwrap();
    let credentials: Credentials = rmp_serde::from_slice(&serialized_credentials).unwrap();
    Ok(credentials)
}
