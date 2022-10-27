use std::path::PathBuf;

use clap::{Parser, Subcommand};

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /*
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(sort, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
*/
    /// Sets a custom config file
    #[arg(short = 'c', long = "config", value_name = "CONFIG", default_value_t = String::from("config.json"))]
    pub config: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigUser {
    add {
        /// username to add
        username: String,
    }
}


#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    user {
        #[command(subcommand)]
        command: ConfigUser,
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage Config
    config {
        #[command(subcommand)]
        command: ConfigAction,
    },
    server {
        #[arg(short = 'p', long = "port", value_name = "PORT", default_value_t = 8000)]
        port: u16,
        #[arg(short = 'H', long = "host", value_name = "HOST", default_value_t = String::from("localhost"))]
        host: String,
    }
}