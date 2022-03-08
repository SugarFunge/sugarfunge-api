use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BundleSchema {
    class_ids: Vec<u64>,
    asset_ids: Vec<Vec<u64>>,
    amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    seed: String,
    class_id: u64,
    asset_id: u64,
    bundle_id: String,
    schema: BundleSchema,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    bundle_id: String,
    who: String,
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleInput {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleOutput {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleInput {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleOutput {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}