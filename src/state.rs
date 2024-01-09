use std::sync::Arc;
use subxt::{client::OnlineClient, PolkadotConfig, backend::legacy::LegacyRpcMethods};

pub type ClientAPI = Arc<OnlineClient<PolkadotConfig>>;
pub type RpcAPI = Arc<LegacyRpcMethods<PolkadotConfig>>;


#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
    pub node_url: String,  // Add this to hold the node URL
    pub rpc: RpcAPI,
}
