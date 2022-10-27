use base64;
use pbkdf2;
use uuid::Uuid;

/*
    TODO: Store token as hash?
    If using salt, we won't be able to use a hash map anymore
    => We will need to test each token
*/

pub fn get_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn storable_token(token: &str) {

}

