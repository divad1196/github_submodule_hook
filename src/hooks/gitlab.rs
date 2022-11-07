use serde;
use serde::{Deserialize};
use super::hooks::{self, split_path_with_ns};

#[derive(Deserialize, Debug)]
pub struct GitlabWebhook {
    pub object_kind: String,
    #[serde(alias = "after")]
    pub hash: Option<String>, // Commit the repository was updated to
    #[serde(alias = "ref")]
    pub branch: Option<String>, // Reference that was updated, e.g. : "refs/heads/master",
    pub project: GitlabWebhookProject,
    pub object_attributes: Option<GitlabWebhookObjectAttributes>,
}
#[derive(Deserialize, Debug)]
pub struct GitlabWebhookProject {
    pub path_with_namespace: String,
}

#[derive(Deserialize, Debug)]
pub struct GitlabWebhookObjectAttributes {
    pub merge_commit_sha: String,
    pub target: GitlabWebhookObjectAttributesTarget,
    pub action: String,
    pub target_branch: String,
}
#[derive(Deserialize, Debug)]
pub struct GitlabWebhookObjectAttributesTarget {
    pub path_with_namespace: String,
}


impl hooks::ToHookTrigger for GitlabWebhook {
    fn as_hook_trigger(self) -> Option<hooks::HookTrigger> {
        match self.object_attributes {
            Some(attrs) => {
                // Merge event
                if self.object_kind != "merge_request" || attrs.action != "merge" {
                    return None;
                }
                let (owner, repo) = split_path_with_ns(&attrs.target.path_with_namespace);
                let hash =  attrs.merge_commit_sha;
                Some(hooks::HookTrigger {
                    owner,
                    repo,
                    branch: attrs.target_branch,
                    hash,
                })
            },
            None => {
                // Push event
                if self.object_kind != "push" {
                    return None;
                }
                let (owner, repo) = split_path_with_ns(&self.project.path_with_namespace);
                if let None = self.hash {
                    return None;
                }
                if let None = self.branch {
                    return None;
                }
                let hash = self.hash.unwrap();
                let branch = match self.branch.unwrap().strip_prefix("refs/heads/") {
                    Some(b) => b.to_owned(),
                    None => {
                        return None;
                    }
                };
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
