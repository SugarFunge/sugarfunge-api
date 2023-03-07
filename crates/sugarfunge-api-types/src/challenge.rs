use crate::primitives::*;
use crate::sugarfunge::runtime_types::functionland_fula::ChallengeState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeStateValue {
    Open,
    Failed,
    Successful,
}

impl Into<ChallengeStateValue> for ChallengeState {
    fn into(self) -> ChallengeStateValue {
        match self {
            ChallengeState::Open => ChallengeStateValue::Open,
            ChallengeState::Failed => ChallengeStateValue::Failed,
            ChallengeState::Successful => ChallengeStateValue::Successful,
        }
    }
}

// GENERATE CHALLENGE STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateChallengeInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateChallengeOutput {
    pub challenger: Account,
    pub challenged: Account,
    pub cid: Cid,
    pub state: ChallengeStateValue,
}

// VERIFY CHALLENGE STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyChallengeInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub cids: Vec<Cid>,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

// CALCULATE AND MINT LABOR TOKENS

#[derive(Serialize, Deserialize, Debug)]
pub struct MintLaborTokensInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintLaborTokensOutput {
    pub account: Account,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: Balance,
}

// Verify Pending Challenge Input

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyPendingChallengeInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyPendingChallengeOutput {
    pub account: Account,
    pub pending: bool,
}

// Verify if there is a File size available to update
#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyFileSizeInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyFileSizeOutput {
    pub account: Account,
    pub cids: Vec<Cid>,
}

// Verify Pending Challenge Input

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvideFileSizeInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub cids: Vec<Cid>,
    pub sizes: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvideFileSizeOutput {
    pub account: Account,
    pub pool_id: PoolId,
    pub cids: Vec<Cid>,
    pub sizes: Vec<u64>,
}
