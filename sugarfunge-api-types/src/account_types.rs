use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountOutput {
    seed: String,
    account: String,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountInput {
    seed: String,
    to: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountOutput {
    from: String,
    to: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceInput {
    account: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceOutput {
    balance: u128,
}