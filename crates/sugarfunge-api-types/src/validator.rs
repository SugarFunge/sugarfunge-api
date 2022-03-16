use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddValidatorInput {
    pub seed: Seed,
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddValidatorOutput {
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorInput {
    pub seed: Seed,
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorOutput {
    pub validator_id: String,
}
