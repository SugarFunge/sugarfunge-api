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
    pub seed: Seed,
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
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
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
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBundles {
    pub bundles: Vec<BundleItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BundleItem {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub bundle_id: BundleId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBundlesData {
    pub bundles: Vec<BundleDataItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BundleDataItem {
    pub bundle_id: BundleId,
    pub creator: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub metadata: serde_json::Value,
    pub schema: BundleSchema,
}
