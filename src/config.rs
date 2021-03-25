use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigEntry {
    pub address: String,
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralConfig {
    pub shop_url: String,
    pub shop_title: String,
    pub logo_url: String,
    pub language: String,
    pub vat: u64,
    pub currency: String,
    pub data_path: String,
    pub stripe_secret_api_key: String,
    pub stripe_public_api_key: String,
    pub sendgrid_sender_email: String,
    pub sendgrid_sender_name: String,
    pub sendgrid_api_key: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub frontend: ConfigEntry,
    pub backend: ConfigEntry,
    pub landing: ConfigEntry,
    pub services: ConfigEntry,
    pub general: GeneralConfig
}



impl Configuration {
    pub fn load(path: &str) -> Result<Configuration, Error> {
        let data = Configuration::read_config(path)
            .unwrap_or(Configuration::read_config("config.json")?);
             
        let config: Configuration = serde_json::from_str(&data).expect("Unable to parse configuration");
        Ok(config)
    }

    fn read_config(path: &str) -> Result<String, Error> {
        if !Path::new(path).exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Unable to find config file {}", path)));
        }
       
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;    
        info!("Using {} configuration", path);            
        Ok(contents)     
    }
}