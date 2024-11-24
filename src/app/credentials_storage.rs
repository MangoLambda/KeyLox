use crate::app::models::vault::Vault;

use rmp_serde;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io::BufReader};

const USE_JSON: bool = true;
const FILE_NAME: &str = "credentials";
pub fn store_vault(vault: &Vault) -> Result<(), Box<dyn Error>> {
    if USE_JSON {
        return store_vault_json(vault);
    } else {
        return store_vault_msgpack(vault);
    }
}

fn store_vault_json(vault: &Vault) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(get_filename())?;
    serde_json::to_writer_pretty(&mut file, &vault)?;
    Ok(())
}

fn store_vault_msgpack(vault: &Vault) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(get_filename())?;
    let serialized_vault = rmp_serde::to_vec(&vault)?;
    file.write_all(serialized_vault.as_slice())?;
    Ok(())
}

pub fn are_credentials_present() -> bool {
    return Path::new(&get_filename()).exists();
}

pub fn load_credentials() -> Result<Option<Vault>, Box<dyn Error>> {
    if !are_credentials_present() {
        return Ok(None);
    }

    if USE_JSON {
        return load_vault_json();
    } else {
        return load_vault_msgpack();
    }
}

fn load_vault_json() -> Result<Option<Vault>, Box<dyn Error>> {
    let file = File::open(get_filename())?;
    let reader = BufReader::new(file);
    let vault: Vault = serde_json::from_reader(reader)?;
    Ok(Some(vault))
}

fn load_vault_msgpack() -> Result<Option<Vault>, Box<dyn Error>> {
    let file = File::open(get_filename())?;
    let reader = BufReader::new(file);
    let vault: Vault = rmp_serde::from_read(reader)?;
    Ok(Some(vault))
}

fn get_filename() -> String {
    if USE_JSON {
        return format!("{}.json", FILE_NAME);
    } else {
        return format!("{}.msgpack", FILE_NAME);
    }
}
