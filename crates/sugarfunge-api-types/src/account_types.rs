use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountOutput1 {
    pub seed: String,
    pub account: String,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountInput {
    pub seed: String,
    pub to: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountOutput {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceInput {
    pub account: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceOutput {
    pub balance: u128,
}
