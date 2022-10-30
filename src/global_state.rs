use crate::hooks;
use crate::access;
use octorust::Client;

pub struct GlobalState {
    pub client: Client,
    pub permissions: access::users::PermissionRegistery,
    pub hooks: hooks::hooks::HookRegistery,
}
