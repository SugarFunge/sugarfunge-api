use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BundleSchema {
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    pub seed: String,
    pub class_id: u64,
    pub asset_id: u64,
    pub schema: BundleSchema,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    pub bundle_id: String,
    pub who: String,
    pub class_id: u64,
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleInput {
    pub seed: String,
    pub from: String,
    pub to: String,
    pub bundle_id: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleOutput {
    pub who: String,
    pub from: String,
    pub to: String,
    pub bundle_id: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleInput {
    pub seed: String,
    pub from: String,
    pub to: String,
    pub bundle_id: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleOutput {
    pub who: String,
    pub from: String,
    pub to: String,
    pub bundle_id: String,
    pub amount: u128,
}
