use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    seed: String,
    class_id: u64,
    metadata: serde_json::Value,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassOutput {
    class_id: u64,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInput {
    seed: String,
    class_id: u64,
    asset_id: u64,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOutput {
    class_id: u64,
    asset_id: u64,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct MintInput {
    seed: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintOutput {
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BurnInput {
    seed: String,
    from: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnOutput {
    from: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceInput {
    account: String,
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesInput {
    account: String,
    class_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesOutput {
    balances: Vec<AssetBalanceItemOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceItemOutput {
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromInput {
    seed: String,
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromOutput {
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}