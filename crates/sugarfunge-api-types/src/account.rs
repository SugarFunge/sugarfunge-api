use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountOutput {
    pub seed: Seed,
    pub account: String,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountInput {
    pub seed: Seed,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountOutput {
    pub from: Account,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceOutput {
    pub balance: Balance,
}
