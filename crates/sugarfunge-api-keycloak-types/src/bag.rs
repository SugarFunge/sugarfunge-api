use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterInput {
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
    pub class_id: ClassId,
    pub owners: Vec<Account>,
    pub shares: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOutput {
    pub bag: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub owners: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SweepInput {
    pub bag: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SweepOutput {
    pub bag: Account,
    pub who: Account,
    pub to: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositInput {
    pub bag: Account,
    pub class_ids: Vec<ClassId>,
    pub asset_ids: Vec<Vec<AssetId>>,
    pub amounts: Vec<Vec<Balance>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositOutput {
    pub bag: Account,
    pub who: Account,
}
