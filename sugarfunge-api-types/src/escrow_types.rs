use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CreateEscrowInput {
    seed: String,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowOutput {
    escrow: String,
    operator: String,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAssetsInput {
    seed: String,
    escrow: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAssetsOutput {
    escrow: String,
    operator: String,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    seed: String,
    escrow: String,
    class_id: u64,
    asset_ids: Vec<u64>,
    amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    escrow: String,
    operator: String,
    owner: String,
}