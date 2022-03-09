use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    pub seed: String,
    pub class_id: u64,
    pub metadata: serde_json::Value,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassOutput {
    pub class_id: u64,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInput {
    pub seed: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOutput {
    pub class_id: u64,
    pub asset_id: u64,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct MintInput {
    pub seed: String,
    pub to: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintOutput {
    pub to: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BurnInput {
    pub seed: String,
    pub from: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnOutput {
    pub from: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceInput {
    pub account: String,
    pub class_id: u64,
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesInput {
    pub account: String,
    pub class_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesOutput {
    pub balances: Vec<AssetBalanceItemOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceItemOutput {
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromInput {
    pub seed: String,
    pub from: String,
    pub to: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromOutput {
    pub from: String,
    pub to: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub amount: u128,
    pub who: String,
}
