use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};


#[derive(Serialize, Deserialize, Debug)]
pub struct BranchPerm(HashSet<String>);

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPerm(HashMap<String, BranchPerm>);

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerPerm(HashMap<String, RepoPerm>);

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPerm(HashMap<String, OwnerPerm>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions(HashMap<String, UserPerm>);

impl Permissions {
    #[allow(unused_variables)]
    pub fn allowed(
        &self,
        user: &str,
        owner: &str,
        repo: &str,
        branch: &str,
        submodule: &str,
    ) -> bool {
        // https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str
        let userperm = self.0.get(&user as &str);
        if let None = userperm {
            return false;
        }

        let ownerperm = userperm.unwrap().0.get(&owner as &str);
        if let None = ownerperm {
            return false;
        }

        let repoperm = ownerperm.unwrap().0.get(&repo as &str);
        if let None = repoperm {
            return false;
        }

        let branchperm = repoperm.unwrap().0.get(&branch as &str);
        if let None = branchperm {
            return false;
        }

        branchperm.unwrap().0.contains(submodule as &str)
    }
}