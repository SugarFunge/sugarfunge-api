use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClassInput {
    pub class_id: ClassId,
    pub metadata: serde_json::Value,
    #[cfg(not(feature = "keycloak"))]
    pub seed: Seed,
    #[cfg(not(feature = "keycloak"))]
    pub owner: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClassOutput {
    pub class_id: ClassId,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassInfoInput {
    pub class_id: ClassId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassInfo {
    pub class_id: ClassId,
    pub owner: Account,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassInfoOutput {
    pub info: Option<ClassInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOutput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfoInput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfo {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfoOutput {
    pub info: Option<AssetInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMetadataInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMetadataOutput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub who: Account,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintInput {
    pub seed: Seed,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintOutput {
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnInput {
    pub seed: Seed,
    pub from: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnOutput {
    pub from: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalanceInput {
    pub account: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalanceOutput {
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalancesInput {
    pub account: Account,
    pub class_id: Option<ClassId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalancesOutput {
    pub balances: Vec<AssetBalanceItemOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalanceItemOutput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFromInput {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFromOutput {
    pub from: Account,
    pub to: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
    pub who: Account,
}
