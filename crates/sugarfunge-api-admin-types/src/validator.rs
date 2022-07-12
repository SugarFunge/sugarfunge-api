use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddValidatorInput {
    pub seed: Seed,
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddValidatorOutput {
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveValidatorInput {
    pub seed: Seed,
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveValidatorOutput {
    pub validator_id: ValidatorId,
}
