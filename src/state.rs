use std::sync::Arc;
use subxt::{DefaultConfig, PolkadotExtrinsicParams};
use sugarfunge_api_types::sugarfunge;

pub type ClientAPI =
    Arc<sugarfunge::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
