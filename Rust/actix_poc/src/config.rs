use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub pool: u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub redis: Redis,
    pub db: Db,
    pub host: String,
    pub port: u16,
}
impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("config.dev.toml")?;
        let config: Config = toml::from_str(&content)?; // Correct error handling
        Ok(config)
    }
}
