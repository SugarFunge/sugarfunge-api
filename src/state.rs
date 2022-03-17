use std::sync::{Arc};
use sugarfunge_api_types::sugarfunge;

pub type ClientAPI = Arc<sugarfunge::RuntimeApi<sugarfunge::DefaultConfig>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
