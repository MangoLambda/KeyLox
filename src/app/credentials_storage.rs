use crate::app::models::vault::Vault;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io::BufReader};

pub fn store_vault(vault: &Vault) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("credentials.msgpack")?;
    let serialized_vault = rmp_serde::to_vec(&vault)?;
    file.write_all(serialized_vault.as_slice())?;
    Ok(())
}

pub fn are_credentials_present() -> bool {
    Path::new("credentials.msgpack").exists()
}

pub fn load_credentials() -> Result<Option<Vault>, Box<dyn Error>> {
    if !Path::new("credentials.msgpack").exists() {
        return Ok(None);
    }

    let file = File::open("credentials.msgpack")?;
    let reader = BufReader::new(file);
    let vault: Vault = rmp_serde::from_read(reader)?;
    Ok(Some(vault))
}
