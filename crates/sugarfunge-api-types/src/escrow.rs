use serde::{Deserialize, Serialize};
use crate::primitives::*;

#[derive(Serialize, Deserialize)]
pub struct RegisterEscrowInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterEscrowOutput {
    pub who: Account,
    pub class_id: ClassId,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub owners: Vec<String>,
    pub shares: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowOutput {
    pub escrow: String,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub owners: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SweepAssetsInput {
    pub seed: Seed,
    pub to: Account,
    pub escrow: String,
}

#[derive(Serialize, Deserialize)]
pub struct SweepAssetsOutput {
    pub escrow: String,
    pub who: Account,
    pub to: Account,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    pub seed: Seed,
    pub escrow: String,
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    pub escrow: String,
    pub who: Account,
}
