use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowInput {
    pub seed: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowOutput {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAssetsInput {
    pub seed: String,
    pub escrow: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAssetsOutput {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    pub seed: String,
    pub escrow: String,
    pub class_id: u64,
    pub asset_ids: Vec<u64>,
    pub amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}
