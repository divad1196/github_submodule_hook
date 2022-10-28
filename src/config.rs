
use serde::{Serialize, Deserialize};
use serde_json;
use anyhow::Result;
use std::io::Read;
use crate::users;

fn default_user_file() -> String {
    "users.txt".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_user_file")]
    pub user_file: String,
    pub token: String,
    pub permissions: users::Permissions,
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