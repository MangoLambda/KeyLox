use crate::app::encryption;
use crate::app::models::credentials::Credentials;
use crate::app::models::vault::Vault;

// TODO: Change salt to object containing info to generate the key material
pub fn encrypt(salt: &Vec<u8>, key: &Vec<u8>, credentials: Credentials) -> Vault {
    // TODO:Error handling
    //let mut serialized_credentials = serde_json::to_string(&credentials).unwrap().into_bytes();
    let mut serialized_credentials = rmp_serde::to_vec(&credentials).unwrap();
    let (nonce, auth_tag) = encryption::encrypt_in_place(key, &[], &mut serialized_credentials);

    Vault::new(&salt, &nonce, &serialized_credentials, &auth_tag)
}

pub fn decrypt(key: &Vec<u8>, vault: Vault) -> Result<Credentials, encryption::DecryptionError> {
    let mut serialized_credentials = vault.protected_data.clone();
    encryption::decrypt_in_place(
        key.as_slice(),
        vault.nonce.as_slice(),
        vault.auth_tag.as_slice(),
        &[],
        &mut serialized_credentials,
    )?;

    // TODO: Error handling
    //let credentials: Credentials = serde_json::from_slice(&serialized_credentials).unwrap();
    let credentials: Credentials = rmp_serde::from_slice(&serialized_credentials).unwrap();
    Ok(credentials)
}
