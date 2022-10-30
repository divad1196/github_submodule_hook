use base64;
use sha2::{Digest, Sha512};
use uuid::Uuid;

/*
    TODO: Store token as hash?
    If using salt, we won't be able to use a hash map anymore
    => We will need to test each token
*/

pub fn get_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn storable_token(token: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(token.as_bytes());
    let res = hasher.finalize();
    base64::encode(res)
}
