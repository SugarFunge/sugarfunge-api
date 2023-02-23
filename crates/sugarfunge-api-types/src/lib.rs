#[subxt::subxt(
    runtime_metadata_path = "sugarfunge_metadata.scale",
    derive_for_type(
        type = "frame_support::traits::tokens::misc::BalanceStatus",
        derive = "serde::Serialize"
    ),
    derive_for_type(type = "pallet_balances::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(type = "sugarfunge_asset::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(type = "sugarfunge_bag::pallet::Event", derive = "serde::Serialize")
)]
pub mod sugarfunge {}
pub mod account;
pub mod asset;
pub mod bag;
pub mod bundle;
pub mod contract;
pub mod fula;
pub mod market;
pub mod pool;
pub mod primitives;
pub mod validator;