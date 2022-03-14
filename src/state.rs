use std::sync::{Arc, Mutex};
use sugarfunge_api_types::sugarfunge;

pub type ClientAPI = Arc<Mutex<sugarfunge::RuntimeApi<sugarfunge::DefaultConfig>>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
