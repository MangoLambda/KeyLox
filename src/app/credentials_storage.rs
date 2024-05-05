use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, io::BufReader};

use crate::models::credentials::Credentials;

pub fn store_credentials(credentials: &Credentials) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("credentials.json")?;
    let credentials_json = serde_json::to_string(credentials)?;
    file.write_all(credentials_json.as_bytes())?;
    Ok(())
}

pub fn load_credentials() -> Result<Option<Credentials>, Box<dyn Error>> {
    if !Path::new("credentials.json").exists() {
        return Ok(None);
    }

    let file = File::open("credentials.json")?;
    let reader = BufReader::new(file);
    let credentials = serde_json::from_reader(reader)?;
    Ok(Some(credentials))
}
