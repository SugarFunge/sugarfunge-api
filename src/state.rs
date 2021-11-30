use crate::sugarfunge;
use std::sync::{Arc, Mutex};

pub type ClientAPI = Arc<Mutex<sugarfunge::RuntimeApi<sugarfunge::DefaultConfig>>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
