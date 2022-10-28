use octorust::Client;
use crate::users;


pub struct GlobalState {
    pub client: Client,
    pub permissions: users::PermissionRegistery,
}
