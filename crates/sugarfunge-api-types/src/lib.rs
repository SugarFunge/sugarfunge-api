#[subxt::subxt(
    runtime_metadata_path = "sugarfunge_metadata.scale",
    derive_for_type(path = "frame_support::traits::tokens::misc::BalanceStatus", derive = "serde::Serialize"),
    derive_for_type(path = "pallet_balances::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(path = "sugarfunge_asset::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(path = "sugarfunge_bag::pallet::Event", derive = "serde::Serialize"),
)]
pub mod sugarfunge {}
pub mod account;
pub mod asset;
pub mod bag;
pub mod bundle;
pub mod market;
pub mod primitives;
pub mod validator;
