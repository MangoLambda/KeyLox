use aegis::aegis256::Aegis256;
use rand::rngs::OsRng;
use rand::Rng;
use std::error::Error;
use std::fmt;

const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 32;
const AUTH_TAG_SIZE: usize = 32;

// Define a custom error type
#[derive(Debug)]
pub struct DecryptionError;

impl fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decryption failed")
    }
}

impl Error for DecryptionError {}

// Returns a nonce and the encrypted data
pub fn encrypt_in_place(
    key: &[u8],
    associated_data: &[u8],
    message: &mut [u8],
) -> ([u8; NONCE_SIZE], [u8; AUTH_TAG_SIZE]) {
    let mut rng = OsRng;
    let nonce: [u8; NONCE_SIZE] = rng.gen(); // 32 bytes of random data
    let key: [u8; KEY_SIZE] = key.try_into().unwrap();

    // TODO: Error handling if key size is incorrect
    let aegis = Aegis256::<AUTH_TAG_SIZE>::new(&key, &nonce);
    let tag = aegis.encrypt_in_place(message, &associated_data);

    return (nonce, tag);
}

pub fn decrypt_in_place(
    key: &[u8],
    nonce: &[u8],
    auth_tag: &[u8],
    associated_data: &[u8],
    message: &mut [u8],
) -> Result<(), DecryptionError> {
    // TODO Error handling
    let key: [u8; KEY_SIZE] = key.try_into().unwrap();
    let nonce: [u8; NONCE_SIZE] = nonce.try_into().unwrap();
    let auth_tag: [u8; AUTH_TAG_SIZE] = auth_tag.try_into().unwrap();
    let aegis = Aegis256::<AUTH_TAG_SIZE>::new(&key, &nonce);

    aegis
        .decrypt_in_place(message, &auth_tag, &associated_data)
        .map_err(|_| DecryptionError)
}
