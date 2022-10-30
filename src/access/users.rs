use crate::access;
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;



#[derive(Debug)]
pub struct UserToken {
    pub map: HashMap<String, String>,
}
impl UserToken {
    pub fn from_str(text: &str) -> Result<UserToken> {
        // https://stackoverflow.com/questions/65373328/convert-key-value-pair-string-to-hashmap-in-one-line
        let map = text
            .split("\n")
            // Split line in 2
            .filter_map(|s| s.split_once("="))
            // Remove whitespace
            .map(|(val, key)| (key.trim().to_string(), val.trim().to_string()))
            .collect();

        Ok(UserToken { map })
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
    pub permissions: access::permissions::Permissions,
}

impl PermissionRegistery {
    #[allow(unused_variables)]
    pub fn allowed(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
        branch: &str,
        submodule: &str,
    ) -> bool {
        let token = access::token::storable_token(&token);
        let user = self.users.map.get(&token as &str);
        if let None = user {
            println!("No User found");
            return false;
        }

        self.permissions
            .allowed(user.unwrap(), &owner, &repo, &branch, &submodule)
    }
}
