use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub class_id: u64,
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDexInput {
    pub seed: String,
    pub exchange_id: u32,
    pub currency: Currency,
    pub asset_class_id: u64,
    pub lp_class_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDexOutput {
    pub exchange_id: u32,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsInput {
    pub seed: String,
    pub exchange_id: u32,
    pub asset_ids: Vec<u64>,
    pub asset_amounts_out: Vec<u128>,
    pub max_currency: u128,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsOutput {
    pub exchange_id: u32,
    pub who: String,
    pub to: String,
    pub asset_ids: Vec<u64>,
    pub asset_amounts_out: Vec<u128>,
    pub currency_amounts_in: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsInput {
    pub seed: String,
    pub exchange_id: u32,
    pub asset_ids: Vec<u64>,
    pub asset_amounts_in: Vec<u128>,
    pub min_currency: u128,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsOutput {
    pub exchange_id: u32,
    pub who: String,
    pub to: String,
    pub asset_ids: Vec<u64>,
    pub asset_amounts_in: Vec<u128>,
    pub currency_amounts_out: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityInput {
    pub seed: String,
    pub to: String,
    pub exchange_id: u32,
    pub asset_ids: Vec<u64>,
    pub asset_amounts: Vec<u128>,
    pub max_currencies: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityOutput {
    pub exchange_id: u32,
    pub who: String,
    pub to: String,
    pub asset_ids: Vec<u64>,
    pub asset_amounts: Vec<u128>,
    pub currency_amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityInput {
    pub seed: String,
    pub to: String,
    pub exchange_id: u32,
    pub asset_ids: Vec<u64>,
    pub liquidities: Vec<u128>,
    pub min_currencies: Vec<u128>,
    pub min_assets: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityOutput {
    pub exchange_id: u32,
    pub who: String,
    pub to: String,
    pub asset_ids: Vec<u64>,
    pub asset_amounts: Vec<u128>,
    pub currency_amounts: Vec<u128>,
}
