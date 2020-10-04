use carapax::{session, Api};

pub struct Context {
    pub api: Api,
    pub session_manager: session::SessionManager<session::backend::fs::FilesystemBackend>,
}
