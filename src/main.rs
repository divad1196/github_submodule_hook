mod submodule;
mod config;
// https://stackoverflow.com/questions/53953795/how-do-i-resolve-the-error-no-module-in-the-root-when-using-a-module-in-rust-2
mod users;  // We need to declare users here to use it in other files
mod global_state;
use global_state::GlobalState;
use octorust::auth::Credentials;
use octorust::Client;

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

// https://api.rocket.rs/master/rocket/struct.Rocket.html
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    /*
        TODO: Init permissions?
    */

    let config = config::Config::from_file("config.json").expect("error");
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

    let _rocket = rocket::build()
        .manage(state)
        .mount("/", routes![update])
        .launch()
        .await?;

    Ok(())
}
