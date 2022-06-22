use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountOutput {
    pub seed: Seed,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountInput {
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountOutput {
    pub from: Account,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceOutput {
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsOutput {
    pub account: Account,
    pub exists: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountOutput {
    pub seed: Seed,
    pub account: Account,
}