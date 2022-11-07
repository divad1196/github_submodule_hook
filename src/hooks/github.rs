use serde;
use serde::{Deserialize};
use super::hooks::{self, HookTrigger};

#[derive(Deserialize, Debug)]
pub struct GithubWebhook {
    #[serde(alias = "after")]
    pub hash: String, // Commit the repository was updated to
    #[serde(alias = "ref")]
    pub branch: String, // Reference that was updated, e.g. : "refs/heads/master",
    pub repository: GithubWebhookRepository,
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookRepository {
    pub name: String,
    pub owner: GithubWebhookRepositoryOwner,
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookRepositoryOwner {
    pub name: String,
}


impl hooks::ToHookTrigger for GithubWebhook {
    fn as_hook_trigger(self) -> Option<hooks::HookTrigger> {
        let branch = self.branch.strip_prefix("refs/heads/");
        if let None = branch {
            return None;
        }
        let branch = branch.unwrap().to_string();
        Some(hooks::HookTrigger {
            owner: self.repository.owner.name,
            repo: self.repository.name,
            branch: branch,
            hash: self.hash,
        })
    }
}
