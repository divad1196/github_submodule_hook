use anyhow::Result;
use octorust::types::{
    GitCreateCommitRequest, GitCreateTagRequestType, GitCreateTreeRequest,
    GitCreateTreeRequestData, GitCreateTreeRequestMode, GitUpdateRefRequest,
};
use octorust::Client;
use serde::{Deserialize, Serialize};

// https://stackoverflow.com/questions/45789854/how-to-update-a-submodule-to-a-specified-commit-via-github-rest-api
pub async fn update_submodule(
    client: &Client,
    owner: &str,
    repo: &str,
    branch: &str,
    submodule_path: &str,
    hash: &str,
) -> Result<()> {
    let repository = client.repos();
    let git = client.git();

    let current_sha = repository
        .get_content_file(owner, repo, submodule_path, branch)
        .await?
        .sha;
    println! {"Currenty Hash: {}", current_sha};
    if current_sha == hash {
        println!("Current hash already correct");
        return Ok(());
    }

    let branch_data = repository
        .get_branch(owner, repo, branch)
        .await
        .expect("test");
    let parent_sha = branch_data.commit.sha;
    println!("Parent SHA: {}", parent_sha);

    let tree_body = GitCreateTreeRequestData {
        base_tree: parent_sha.clone(),
        tree: vec![GitCreateTreeRequest {
            content: "".to_string(),
            mode: Some(GitCreateTreeRequestMode::SubmoduleCommit),
            path: submodule_path.to_string(),
            sha: hash.to_string(),
            type_: Some(GitCreateTagRequestType::Commit),
        }],
    };
    let tree_data = git.create_tree(owner, repo, &tree_body).await?;
    println!("Tree SHA: {}", tree_data.sha);

    let commit_body = GitCreateCommitRequest {
        author: None,
        committer: None,
        message: format!(
            "Automatic update submodule {} to hash '{}'",
            submodule_path, hash
        ),
        parents: vec![parent_sha.clone()],
        signature: "".to_string(),
        tree: tree_data.sha,
    };
    let commit_data = git.create_commit(owner, repo, &commit_body).await?;
    println!("Commit SHA: {}", commit_data.sha);

    let ref_body = GitUpdateRefRequest {
        force: None,
        sha: commit_data.sha,
    };
    let ref_data = git
        .update_ref(owner, repo, &format!("heads/{}", branch), &ref_body)
        .await?;
    println!("Ref SHA: {}", ref_data.ref_);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmoduleUpdate {
    owner: String,
    repo: String,
    branch: String,
    submodule_path: String,
    hash: String,
}

impl SubmoduleUpdate {
    pub fn new(
        owner: &str,
        repo: &str,
        branch: &str,
        submodule_path: &str,
        hash: &str,
    ) -> SubmoduleUpdate {
        SubmoduleUpdate {
            owner: owner.to_string(),
            repo: repo.to_string(),
            branch: branch.to_string(),
            submodule_path: submodule_path.to_string(),
            hash: hash.to_string(),
        }
    }
    pub async fn update_as(&self, client: &Client) -> Result<()> {
        update_submodule(
            client,
            &self.owner,
            &self.repo,
            &self.branch,
            &self.submodule_path,
            &self.hash,
        )
        .await
    }
}
