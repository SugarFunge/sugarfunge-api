use std::sync::Arc;
use subxt::{client::OnlineClient, PolkadotConfig};

pub type ClientAPI = Arc<OnlineClient<PolkadotConfig>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
