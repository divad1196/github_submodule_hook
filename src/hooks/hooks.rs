use crate::submodule::SubmoduleUpdateData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BranchHook(Vec<SubmoduleUpdateData>);

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoHooh(HashMap<String, BranchHook>);

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerHook(HashMap<String, RepoHooh>);

#[derive(Serialize, Deserialize, Debug)]
pub struct HookRegistery(HashMap<String, OwnerHook>);

impl HookRegistery {
    #[allow(unused_variables)]
    pub fn get(&self, owner: &str, repo: &str, branch: &str) -> Option<&Vec<SubmoduleUpdateData>> {
        let ownerhook = self.0.get(&owner as &str);
        if let None = ownerhook {
            return None;
        }

        let repohook = ownerhook.unwrap().0.get(&repo as &str);
        if let None = repohook {
            return None;
        }

        let branchhook = repohook.unwrap().0.get(&branch as &str);
        if let None = branchhook {
            return None;
        }
        Some(&branchhook.unwrap().0)
    }
}

#[derive(Debug)]
pub struct HookTrigger {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub hash: String
}

pub trait ToHookTrigger {
    fn as_hook_trigger(self) -> Option<HookTrigger>;
}
