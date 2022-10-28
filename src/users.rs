
use anyhow::Result;
use std::io::BufRead;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use crate::token;

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
    pub fn allowed(&self, user: &str, owner: &str, repo: &str, branch: &str, submodule: &str) -> bool {
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

#[derive(Debug)]
pub struct UserToken {
    map: HashMap<String, String>
}

impl UserToken {
    pub fn from_str(text: &str) -> Result<UserToken> {

        // https://stackoverflow.com/questions/65373328/convert-key-value-pair-string-to-hashmap-in-one-line
        let map = text.split("\n")
        // Split line in 2
        .filter_map(
            |s| s.split_once("=")
        )
        // Remove whitespace
        .map(
            |(val, key)|
            (key.trim().to_string(), val.trim().to_string())
        )
        .collect();

        Ok(UserToken {
            map
        })
    }
    pub fn from_file(filename: &str) -> Result<UserToken> {
        let file = std::fs::File::open(filename)?;
        UserToken::from_reader(&mut std::io::BufReader::new(file))
    }
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Result<UserToken> {
        let mut text = "".to_string();
        reader.read_to_string(&mut text)?;
        return UserToken::from_str(&text);
    }
}

pub struct PermissionRegistery {
    pub users: UserToken,
    pub permissions: Permissions,
}

impl PermissionRegistery {
    #[allow(unused_variables)]
    pub fn allowed(&self, token: &str, owner: &str, repo: &str, branch: &str, submodule: &str) -> bool {
        let token = token::storable_token(&token);
        let user = self.users.map.get(&token as &str);
        if let None = user {
            println!("No User found");
            return false;
        }
        
        self.permissions.allowed(user.unwrap(), &owner, &repo, &branch, &submodule)

    }
}