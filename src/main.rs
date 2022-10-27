mod submodule;
mod config;
// https://stackoverflow.com/questions/53953795/how-do-i-resolve-the-error-no-module-in-the-root-when-using-a-module-in-rust-2
mod users;  // We need to declare users here to use it in other files
mod cli;
mod global_state;
mod token;
use clap::Parser;
use global_state::GlobalState;
use octorust::auth::Credentials;
use octorust::Client;
use std::io::{Write};

// use rocket::config::{ConfigBuilder};

#[macro_use]
extern crate rocket;
use rocket::State;

#[post("/update/<owner>/<repo>/<branch>/<submodule>/<hash>?<token>")]
async fn update(
    state: &State<GlobalState>,
    owner: &str,
    repo: &str,
    branch: &str,
    submodule: &str,
    hash: &str,
    token: Option<String>
) -> &'static str {

    if token.is_none() {
        return "Missing token";
    }
    let token = token.unwrap();
    println!("Token: {}", token);

    let client = &state.client;
    if !state.permissions.allowed( &token, owner, repo, branch, submodule) {
        return "You are not allowed to do this action";
    }
    let result = submodule::update_submodule(&client, owner, repo, branch, submodule, hash).await;
    match result {
        Ok(_) => "Update Success",
        Err(_) => "Error on update",
    }
}

async fn run_server(config: config::Config) -> Result<(), rocket::Error> {

    let registry = users::PermissionRegistery {
        users: users::UserToken::from_file(&config.user_file).expect("Failed to load users"),
        permissions: config.permissions,
    };
    println!("{:?}", registry.users);
    println!("{:?}", registry.permissions);

    // https://api.rocket.rs/v0.4/rocket/struct.State.html
    let token = std::fs::read_to_string("token.txt").expect("File does not exists");

    let creds = Credentials::Token(token);
    let client = Client::new("odoo-hook", creds).unwrap();
    let state = GlobalState {
        client: client,
        permissions: registry,
    };
/* 
    let rocket_config = Config::
        .address("localhost")
        .port(8000)
        .finalize()?;
*/
    let _rocket = rocket::build()  // ::custom(rocket_config)
        .manage(state)
        .mount("/", routes![update])
        .launch()
        .await?;
    Ok(())
}

fn add_user(config: config::Config, username: &String) {
    println!("Adding user {}", username);
    let token = token::get_token();
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
    println!("{:?}", cli);

    let config = config::Config::from_file(&cli.config).expect("error");

    match cli.command {
        cli::Commands::server { port, host } => {
            run_server(config).await?;
        },
        cli::Commands::config { command } => {
            match command {
                cli::ConfigAction::user { command } => {
                    match command {
                        cli::ConfigUser::add { username } => add_user(config, &username)
                    };
                }
            };
        }
    };

    Ok(())
}
