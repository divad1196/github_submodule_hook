use crate::hooks;
use crate::access;
use crate::submodule;

use octorust::Client;
use anyhow::{Result, anyhow};

pub struct GlobalState {
    pub client: Client,
    pub permissions: access::users::PermissionRegistery,
    pub hooks: hooks::hooks::HookRegistery,
}

impl GlobalState {
    pub fn get_user(&self, token: &str) -> Result<&String> {
        // Retrieve user
        let token = access::token::storable_token(&token);
        match self.permissions.users.map.get(&token as &str) {
            Some(user) => Ok(user),
            None => Err(anyhow!("No User found"))
        }
    }

    async fn _trigger_hooks(&self, token: &str, owner: &str, repo: &str, branch: &str, hash: &str) -> &'static str {
        // Get list of ooks
        let hooks = self.hooks.get(
            owner,
            repo,
            branch,
        );
        if let None = hooks {
            return "Nothing to be done";
        }
        let hooks = hooks.unwrap();
        println!("Hooks found {:?}", hooks);

        // Retrieve client
        let client = &self.client;

        // Retrieve user
        let user = self.get_user(&token);
        if let Err(e) = user {
            println!("{}", e);
            return "User not found";
        }
        let user = user.unwrap();

        // Trigger allowed hooks
        for h in hooks {
            let allowed =
                self
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
                    &hash,
                )
                .await;
            }
        };
        "Ok"
    }
    pub async fn trigger_hooks(&self, token: &str, trigger: hooks::hooks::HookTrigger) -> &'static str {
        println!("Hook Trigger: {:?}", trigger);
        self._trigger_hooks(token, &trigger.owner, &trigger.repo, &trigger.branch, &trigger.hash).await
    }
}
