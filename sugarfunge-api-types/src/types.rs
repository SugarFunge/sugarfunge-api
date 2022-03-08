use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountOutput {
    seed: String,
    account: String,
}
