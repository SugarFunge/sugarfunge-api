#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
pub mod account;
pub mod asset;
pub mod bag;
pub mod bundle;
pub mod config;
pub mod market;
pub mod primitives;
pub mod user;
pub mod validator;
