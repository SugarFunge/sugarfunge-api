use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BundleSchema {
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub bundle_id: String,
    pub schema: BundleSchema,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    pub bundle_id: String,
    pub who: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleInput {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub bundle_id: String,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleOutput {
    pub who: Account,
    pub from: Account,
    pub to: Account,
    pub bundle_id: String,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleInput {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub bundle_id: String,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleOutput {
    pub who: Account,
    pub from: Account,
    pub to: Account,
    pub bundle_id: String,
    pub amount: Balance,
}
