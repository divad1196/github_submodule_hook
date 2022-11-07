mod config;
mod submodule;
// https://stackoverflow.com/questions/53953795/how-do-i-resolve-the-error-no-module-in-the-root-when-using-a-module-in-rust-2
mod cli;
mod global_state;
mod hooks;
mod server;
mod access;
mod constants;

use anyhow::Result;
use clap::Parser;
use std::io::Write;

#[macro_use]
extern crate rocket;
// use rocket_contrib::json::Json;

// use rocket::config::{ConfigBuilder};

fn add_user(config: config::Config, username: &String) {
    println!("Adding user {}", username);
    let token = access::token::get_token();
    println!(
        "Here is your token, save it now for you won't see it again:\n{}",
        token
    );

    let token = access::token::storable_token(&token);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(config.user_file)
        .unwrap();
    if let Err(e) = writeln!(file, "{} = {}", username, token) {
        eprintln!("Couldn't write to file: {}", e);
    }
    // config.user_file
}
// https://api.rocket.rs/master/rocket/struct.Rocket.html
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = cli::Cli::parse();

    // Userfile is adjacent to config file
    let config = cli.get_config();

    match cli.command {
        cli::Commands::Server { port, host } => {
            server::server::run_server(config.unwrap(), port, host).await?;
        }
        cli::Commands::Config { command } => {
            match command {
                cli::ConfigAction::User { command } => {
                    match command {
                        cli::ConfigUser::Add { username } => add_user(config.unwrap(), &username),
                    };
                }
                cli::ConfigAction::Make {} => config::make_config(),
            };
        } /*cli::Commands::Completion { shell } => {
              println!("Sorry, this is currently not implemented");
          }*/
    };

    Ok(())
}
