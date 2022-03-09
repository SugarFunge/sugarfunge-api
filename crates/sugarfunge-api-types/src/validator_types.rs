use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddValidatorInput {
    seed: String,
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddValidatorOutput {
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorInput {
    seed: String,
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorOutput {
    validator_id: String,
}
