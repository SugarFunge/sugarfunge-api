use std::sync::Arc;
use subxt::{DefaultConfig, SubstrateExtrinsicParams};
use sugarfunge_api_types::sugarfunge;

pub type ClientAPI =
    Arc<sugarfunge::RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>>;

#[derive(Clone)]
pub struct AppState {
    pub api: ClientAPI,
}
