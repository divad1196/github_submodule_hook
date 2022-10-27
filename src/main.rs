mod submodule;
use octorust::auth::Credentials;
use octorust::Client;

#[macro_use]
extern crate rocket;
use rocket::State;

#[post("/update/<owner>/<repo>/<branch>/<submodule>/<hash>")]
async fn update(
    state: &State<GlobalState>,
    owner: &str,
    repo: &str,
    branch: &str,
    submodule: &str,
    hash: &str,
) -> &'static str {
    let client = &state.client;
    let result = submodule::update_submodule(&client, owner, repo, branch, submodule, hash).await;
    match result {
        Ok(_) => "Update Success",
        Err(_) => "Error on update",
    }
}

struct GlobalState {
    client: Client,
}

// https://api.rocket.rs/master/rocket/struct.Rocket.html
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    /*
        TODO: Init permissions?
    */

    // https://api.rocket.rs/v0.4/rocket/struct.State.html
    let token = std::fs::read_to_string("token.txt").expect("File does not exists");

    let creds = Credentials::Token(token);
    let client = Client::new("odoo-hook", creds).unwrap();
    let state = GlobalState { client: client };

    let _rocket = rocket::build()
        .manage(state)
        .mount("/", routes![update])
        .launch()
        .await?;

    Ok(())
}
