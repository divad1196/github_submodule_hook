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
use std::io::Write;

#[macro_use]
extern crate rocket;
use rocket::State;

// use rocket::config::{ConfigBuilder};
const EXAMPLE_CONFIG: &str = std::include_str!("../config_example.json");

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

async fn run_server(config: config::Config, port: Option<u16>, host: Option<String>) -> Result<(), rocket::Error> {

    let registry = users::PermissionRegistery {
        users: users::UserToken::from_file(&config.user_file).expect("Failed to load users"),
        permissions: config.permissions,
    };

    // https://api.rocket.rs/v0.4/rocket/struct.State.html
    let token = std::fs::read_to_string("token.txt").expect("File does not exists");

    let creds = Credentials::Token(token);
    let client = Client::new("odoo-hook", creds).unwrap();
    let state = GlobalState {
        client: client,
        permissions: registry,
    };
    
    // https://rocket.rs/v0.5-rc/guide/configuration/#custom-providers
    let mut figment = rocket::Config::figment();
    if let Some(port) = port {
        figment = figment.merge(("port", port));
    }
    if let Some(host) = host {
        figment= figment.merge(("address", host));
    }
    // .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    let figment = figment;

    let _rocket = rocket::custom(figment)  // :
        .manage(state)
        .mount("/", routes![update])
        .launch()
        .await?;
    Ok(())
}

fn add_user(config: config::Config, username: &String) {
    println!("Adding user {}", username);
    let token = token::get_token();
    println!("Here is your token, save it now for you won't see it again:\n{}", token);

    let token = token::storable_token(&token);
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

fn make_config() {
    let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open("config.json").unwrap();
    if let Err(e) = writeln!(file, "{}", EXAMPLE_CONFIG) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

// https://api.rocket.rs/master/rocket/struct.Rocket.html
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let cli = cli::Cli::parse();

    // Userfile is adjacent to config file
    let config = cli.get_config();


    match cli.command {
        cli::Commands::Server { port, host } => {
            run_server(config.unwrap(), port, host).await?;
        },
        cli::Commands::Config { command } => {
            match command {
                cli::ConfigAction::User { command } => {
                    match command {
                        cli::ConfigUser::Add { username } => add_user(config.unwrap(), &username)
                    };
                },
                cli::ConfigAction::Make {  } => { make_config() }
            };
        },
        /*cli::Commands::Completion { shell } => {
            println!("Sorry, this is currently not implemented");
        }*/
    };

    Ok(())
}
