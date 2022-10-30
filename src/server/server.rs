use octorust::auth::Credentials;
use octorust::Client;
use rocket::State;
use rocket::figment::Figment;

use crate::config;
use crate::global_state::GlobalState;
use crate::submodule;
use crate::access;
use crate::server::types::{Json, JsonBody};
use crate::hooks::github::GithubWebhook;

#[post("/update/<owner>/<repo>/<branch>/<submodule>/<hash>?<token>")]
async fn update(
    state: &State<GlobalState>,
    owner: &str,
    repo: &str,
    branch: &str,
    submodule: &str,
    hash: &str,
    token: Option<String>,
) -> &'static str {
    if token.is_none() {
        return "Missing token";
    }
    let token = token.unwrap();
    println!("Token: {}", token);

    let client = &state.client;
    if !state
        .permissions
        .allowed(&token, owner, repo, branch, submodule)
    {
        return "You are not allowed to do this action";
    }
    let result = submodule::update_submodule(&client, owner, repo, branch, submodule, hash).await;
    match result {
        Ok(_) => "Update Success",
        Err(e) => {
            eprintln!("{}", e);
            "Error on update"
        }
    }
}
/*
#[post("/github/webhook", format = "application/json", data = "<data>")]
async fn github_webhook(data: Data<'_>) -> &'static str {
    let json = match data_to_json(data).await {
        Ok(d) => d,
        Err(e) => {
            return "error occured";
        }
    };
    println!("{:?}", json);
    "Ok"
}*/

#[post("/github/webhook2", format = "application/json", data = "<data>")]
async fn github_webhook2(data: JsonBody) -> &'static str {
    data.print();
    "Ok"
}
#[post(
    "/github/webhook?<token>",
    format = "application/json",
    data = "<data>"
)]
async fn github_webhook(
    state: &State<GlobalState>,
    data: Json<GithubWebhook>,
    token: Option<String>,
) -> &'static str {
    println!("{:?}", data.data);
    if token.is_none() {
        return "Missing token";
    }
    let token = token.unwrap();
    println!("Token: {}", token);

    let data = data.data;

    let hooks = state.hooks.get(
        &data.repository.owner.name,
        &data.repository.name,
        &data.branch,
    );
    if let None = hooks {
        return "Nothing to be done";
    }
    let hooks = hooks.unwrap();
    println!("Hooks found {:?}", hooks);

    let client = &state.client;

    let token = access::token::storable_token(&token);
    let user = state.permissions.users.map.get(&token as &str);
    if let None = user {
        println!("No User found");
        return "No User found";
    }
    let user = user.unwrap();
    for h in hooks {
        let allowed =
            state
                .permissions
                .permissions
                .allowed(user, &h.owner, &h.repo, &h.branch, &h.submodule);
        if allowed {
            let _result = submodule::update_submodule(
                &client,
                &h.owner,
                &h.repo,
                &h.branch,
                &h.submodule,
                &data.hash,
            )
            .await;
        }
    }
    "Ok"
}


fn get_config(port: Option<u16>, host: Option<String>) ->  Figment {
    // https://rocket.rs/v0.5-rc/guide/configuration/#custom-providers
    let mut figment = rocket::Config::figment();
    // let mut figment = rocket::build().figment().clone();
    if let Some(port) = port {
        figment = figment.merge(("port", port));
    }
    if let Some(host) = host {
        figment = figment.merge(("address", host));
    }
    // .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    figment
}

pub async fn run_server(
    config: config::Config,
    port: Option<u16>,
    host: Option<String>,
) -> Result<(), rocket::Error> {
    let registry = access::users::PermissionRegistery {
        users: access::users::UserToken::from_file(&config.user_file).expect("Failed to load users"),
        permissions: config.permissions,
    };

    // https://api.rocket.rs/v0.4/rocket/struct.State.html
    // let token = std::fs::read_to_string("token.txt").expect("File does not exists");
    let token = config.token;

    let creds = Credentials::Token(token);
    let client = Client::new("odoo-hook", creds).unwrap();
    let state = GlobalState {
        client: client,
        permissions: registry,
        hooks: config.hooks,
    };

    let figment = get_config(port, host);
    let _rocket = rocket::custom(figment) // :
        .manage(state)
        .mount("/", routes![update, github_webhook, github_webhook2])
        .launch()
        .await?;
    Ok(())
}
