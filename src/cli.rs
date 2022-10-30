use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
// use clap_complete::{generate, Generator, Shell};
use crate::config;
use home;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(author = "David Gallay (divad1196)", version = "0.1", about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short = 'c', long = "config", value_name = "CONFIG")]
    //  default_value_t = String::from("config.json")
    config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum ConfigUser {
    /// Add a user and generate its token
    Add {
        /// username to add
        username: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Manage users
    User {
        #[command(subcommand)]
        command: ConfigUser,
    },
    /// Create a configuration skelton
    Make {},
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage Config
    Config {
        #[command(subcommand)]
        command: ConfigAction,
    },
    /// Start the service's server
    Server {
        #[arg(short = 'p', long = "port", value_name = "PORT")]
        port: Option<u16>,
        #[arg(short = 'H', long = "host", value_name = "HOST")]
        host: Option<String>,
    },
    /*Completion {
        shell: Shell
    }*/
}

/*
pub fn print_completions(shell: &Shell, cmd: &Command) {
    // The library requires a Command struct, We only have a Parser
    shell.generate(&cmd, &mut std::io::stdout());
}*/

impl Cli {
    pub fn get_config(&self) -> Result<config::Config> {
        let config_path = self.get_config_path()?;
        println!("Config path: {}", config_path);

        let mut config = config::Config::from_file(&config_path)?;

        config.user_file = PathBuf::from(&config_path)
            .parent()
            .unwrap()
            .join(config.user_file)
            .to_str()
            .expect("Error when accessing userfile")
            .to_string();

        Ok(config)
    }
    fn get_config_path(&self) -> Result<String> {
        // From Cli
        if let Some(conf) = &self.config {
            return Ok(conf.clone());
        }

        // In current directory
        if let Ok(path) = std::env::current_dir() {
            let config = path.join("config.json");
            if config.exists() {
                return Ok(config.to_str().unwrap().to_string());
            }
        }

        // From environment variable
        if let Ok(conf) = std::env::var("GITHUB_SUBMODULE_HOOK_CONFIG") {
            return Ok(conf);
        }

        // In ~/.github_submodule_hook
        if let Some(path) = home::home_dir() {
            let config = path.join(".github_submodule_hook").join("config.json");
            if config.exists() {
                return Ok(config.to_str().unwrap().to_string());
            }
        }

        // In /etc/github_submodule_hook
        let config = PathBuf::from("/etc/github_submodule_hook/config.json");
        if config.exists() {
            return Ok(config.to_str().unwrap().to_string());
        }

        // Adjacent to executable
        if let Ok(path) = std::env::current_exe() {
            if let Some(parent_path) = path.parent() {
                let config = parent_path.join("config.json");
                if config.exists() {
                    return Ok(config.to_str().unwrap().to_string());
                }
            }
        }
        Err(anyhow!("No valid configuration found"))
    }
}
