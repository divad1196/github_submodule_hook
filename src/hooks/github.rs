use serde;
use serde::{Deserialize};
use super::hooks::{self, split_path_with_ns};

#[derive(Deserialize, Debug)]
pub struct GithubWebhook {
    #[serde(alias = "after")]
    pub hash: Option<String>, // Commit the repository was updated to
    #[serde(alias = "ref")]
    pub branch: Option<String>, // Reference that was updated, e.g. : "refs/heads/master",
    pub repository: GithubWebhookRepository,
    pub pull_request: Option<GithubWebhookPullRequest>,
    pub deleted: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookRepository {
    pub name: String,
    pub owner: GithubWebhookRepositoryOwner,
    pub full_name: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookRepositoryOwner {
    pub name: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct GithubWebhookPullRequest {
    pub base: GithubWebhookPullRequestBase,
    pub head: GithubWebhookPullRequestHead,
    pub merged: bool
}

#[derive(Deserialize, Debug)]
pub struct GithubWebhookPullRequestHead {
    pub sha: String,
    pub merge_commit_sha: Option<String>,

}
#[derive(Deserialize, Debug)]
pub struct GithubWebhookPullRequestBase {
    #[serde(alias = "ref")]
    pub branch: String,

}

impl hooks::ToHookTrigger for GithubWebhook {
    fn as_hook_trigger(self) -> Option<hooks::HookTrigger> {
        match self.pull_request {
            Some(pr) => {
                // Pull request, Nb: Github is making a push after validating the merge, we don't really need this part.
                if !pr.merged {return  None;}

                let hash = pr.head.merge_commit_sha;
                if let None = hash { return None; }
                let hash = hash.unwrap();

                let branch = pr.base.branch;
                let (owner, repo) = split_path_with_ns(&self.repository.full_name);
                Some(hooks::HookTrigger {
                    owner,
                    repo,
                    branch,
                    hash,
                })
            },
            None => {
                // Push
                if self.deleted.unwrap_or(false) {return None;}
                let branch = self.branch.and_then(
                    |b| b.strip_prefix("refs/heads/").map(|s| s.to_string())
                );
                if let None = branch {
                    return None;
                }
                let branch = branch.unwrap();
                let (owner, repo) = split_path_with_ns(&self.repository.full_name);
                if let None = self.hash { return None;}
                let hash = self.hash.unwrap();
                Some(hooks::HookTrigger {
                    owner,
                    repo,
                    branch,
                    hash,
                })
            }
        }
    }
}
