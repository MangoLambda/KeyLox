use crate::models::vault::Vault;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io::BufReader};

pub fn store_vault(vault: &Vault) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("credentials.bin")?;
    let serialized_vault = rmp_serde::to_vec(&vault)?;
    //let credentials_json = serde_json::to_string(credentials)?;
    file.write_all(serialized_vault.as_slice())?;
    Ok(())
}

pub fn load_credentials() -> Result<Option<Vault>, Box<dyn Error>> {
    if !Path::new("credentials.bin").exists() {
        return Ok(None);
    }

    let file = File::open("credentials.bin")?;
    let reader = BufReader::new(file);
    let vault: Vault = rmp_serde::from_read(reader)?;
    Ok(Some(vault))
}
