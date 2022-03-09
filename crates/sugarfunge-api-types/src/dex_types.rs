use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Currency {
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDexInput {
    seed: String,
    exchange_id: u32,
    currency: Currency,
    asset_class_id: u64,
    lp_class_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDexOutput {
    exchange_id: u32,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    max_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    currency_amounts_in: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    min_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    currency_amounts_out: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    max_currencies: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    liquidities: Vec<u128>,
    min_currencies: Vec<u128>,
    min_assets: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}
