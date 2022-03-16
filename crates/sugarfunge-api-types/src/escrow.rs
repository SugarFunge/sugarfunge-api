use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterEscrowInput {
    pub seed: String,
    pub class_id: u64,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterEscrowOutput {
    pub who: String,
    pub class_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowInput {
    pub seed: String,
    pub class_id: u64,
    pub owners: Vec<String>,
    pub shares: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowOutput {
    pub escrow: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub owners: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SweepAssetsInput {
    pub seed: String,
    pub to: String,
    pub escrow: String,
}

#[derive(Serialize, Deserialize)]
pub struct SweepAssetsOutput {
    pub escrow: String,
    pub who: String,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    pub seed: String,
    pub escrow: String,
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    pub escrow: String,
    pub who: String,
}
