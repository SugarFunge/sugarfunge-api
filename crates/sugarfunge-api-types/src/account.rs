use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountOutput {
    pub seed: Seed,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountInput {
    pub seed: Seed,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountOutput {
    pub from: Account,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceOutput {
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsOutput {
    pub account: Account,
    pub exists: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountOutput {
    pub seed: Seed,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Refund {
    pub seed: String,
    pub amount: u128,
}
