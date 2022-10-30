use crate::hooks;
use crate::access;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Read, Write};
const EXAMPLE_CONFIG: &str = std::include_str!("../config_example.json");

fn default_user_file() -> String {
    "users.txt".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_user_file")]
    pub user_file: String,
    pub token: String,
    pub permissions: access::permissions::Permissions,
    pub hooks: hooks::hooks::HookRegistery,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Config> {
        let file = std::fs::File::open(filename)?;
        Config::from_reader(&file)
    }
    pub fn from_reader<R: Read>(reader: R) -> Result<Config> {
        Ok(serde_json::from_reader::<R, Config>(reader)?)
    }
}

pub fn make_config() {
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("config.json")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", EXAMPLE_CONFIG) {
        eprintln!("Couldn't write to file: {}", e);
    }
}