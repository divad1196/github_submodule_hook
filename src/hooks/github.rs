use serde;
use serde::{Deserialize};

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