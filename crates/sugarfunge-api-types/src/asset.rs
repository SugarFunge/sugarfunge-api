use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: serde_json::Value,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassOutput {
    pub class_id: ClassId,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOutput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct MintInput {
    pub seed: Seed,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct MintOutput {
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct BurnInput {
    pub seed: Seed,
    pub from: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct BurnOutput {
    pub from: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceInput {
    pub account: String,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    pub amount: Balance,
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
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromInput {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromOutput {
    pub from: Account,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}
