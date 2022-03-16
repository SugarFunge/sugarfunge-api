#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
pub mod account;
pub mod asset;
pub mod bundle;
pub mod currency;
pub mod dex;
pub mod escrow;
pub mod market;
pub mod primitives;
pub mod validator;
