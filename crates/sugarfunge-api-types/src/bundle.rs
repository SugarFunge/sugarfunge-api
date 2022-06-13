use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BundleSchema {
    pub class_ids: Vec<ClassId>,
    pub asset_ids: Vec<Vec<AssetId>>,
    pub amounts: Vec<Vec<Balance>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterBundleInput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub schema: BundleSchema,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterBundleOutput {
    pub bundle_id: BundleId,
    pub who: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintBundleInput {
    pub bundle_id: BundleId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintBundleOutput {
    pub who: Account,
    pub from: Account,
    pub to: Account,
    pub bundle_id: BundleId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnBundleInput {
    pub bundle_id: BundleId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnBundleOutput {
    pub who: Account,
    pub from: Account,
    pub to: Account,
    pub bundle_id: BundleId,
    pub amount: Balance,
}
