use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterOutput {
    pub who: Account,
    pub class_id: ClassId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInput {
    pub seed: Seed,
    pub class_id: ClassId,
    pub owners: Vec<String>,
    pub shares: Vec<u128>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOutput {
    pub bag: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub owners: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SweepInput {
    pub seed: Seed,
    pub bag: Account,
    pub to: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SweepOutput {
    pub bag: Account,
    pub who: Account,
    pub to: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositInput {
    pub seed: Seed,
    pub bag: Account,
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositOutput {
    pub bag: Account,
    pub who: Account,
}
