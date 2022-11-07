use octorust::auth::Credentials;
use octorust::Client;
use rocket::{State, Route};
use rocket::figment::Figment;

use crate::config;
use crate::global_state::GlobalState;
use crate::hooks::hooks::ToHookTrigger;
use crate::submodule;
use crate::access;
use crate::server::types::{OptionalJson, JsonBody};
use crate::hooks::github::GithubWebhook;
use crate::hooks::gitlab::GitlabWebhook;

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

#[post("/dev", format = "application/json", data = "<data>")]
async fn dev(data: JsonBody) -> &'static str {
    data.print();
    "Ok"
}


#[post(
    "/gitlab/webhook?<token>",
    format = "application/json",
    data = "<data>"
)]
async fn gitlab_webhook(
    state: &State<GlobalState>,
    data: OptionalJson<GitlabWebhook>,
    token: Option<String>,
) -> &'static str {
    println!("{:?}", data.data);
    if token.is_none() {
        return "Missing token";
    }
    let token = token.unwrap();
    println!("Token: {}", token);
    if data.data.is_none() {
        return "Nothing to do";
    }

    let trigger = data.data.unwrap().as_hook_trigger();
    if let None = trigger {
        return "Nothing to do"
    }
    let trigger = trigger.unwrap();
    state.trigger_hooks(&token, trigger).await
}

#[post(
    "/github/webhook?<token>",
    format = "application/json",
    data = "<data>"
)]
async fn github_webhook(
    state: &State<GlobalState>,
    data: OptionalJson<GithubWebhook>,
    token: Option<String>,
) -> &'static str {
    println!("{:?}", data.data);
    if token.is_none() {
        return "Missing token";
    }
    let token = token.unwrap();
    println!("Token: {}", token);

    if data.data.is_none() {
        return "Nothing to do";
    }

    let trigger = data.data.unwrap().as_hook_trigger();
    if let None = trigger {
        return "Nothing to do"
    }
    let trigger = trigger.unwrap();
    state.trigger_hooks(&token, trigger).await
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

// dev route only available for development
// https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
#[cfg(debug_assertions)]
fn get_routes() -> Vec<Route> {
    routes![update, github_webhook, gitlab_webhook, dev]
}

#[cfg(not(debug_assertions))]
fn get_routes() -> Vec<Route> {
    routes![update, github_webhook, gitlab_webhook,]
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
        client,
        permissions: registry,
        hooks: config.hooks,
    };

    let figment = get_config(port, host);
    let _rocket = rocket::custom(figment) // :
        .manage(state)
        .mount("/", get_routes())
        .launch()
        .await?;
    Ok(())
}
