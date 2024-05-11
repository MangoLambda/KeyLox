use crate::models::vault::Vault;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io::BufReader};

pub fn store_vault(vault: &Vault) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("credentials.json")?;
    //let serialized_vault = rmp_serde::to_vec(&vault)?;
    let credentials_json = serde_json::to_string(vault)?;
    file.write_all(&credentials_json.into_bytes())?;
    //file.write_all(serialized_vault.as_slice())?;
    Ok(())
}

pub fn load_credentials() -> Result<Option<Vault>, Box<dyn Error>> {
    if !Path::new("credentials.json").exists() {
        return Ok(None);
    }

    let file = File::open("credentials.json")?;
    let reader = BufReader::new(file);
    //let vault: Vault = rmp_serde::from_read(reader)?;
    let vault: Vault = serde_json::from_reader(reader)?;
    Ok(Some(vault))
}
