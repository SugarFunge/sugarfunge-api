use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddValidatorInput {
    pub seed: String,
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddValidatorOutput {
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorInput {
    pub seed: String,
    pub validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorOutput {
    pub validator_id: String,
}
