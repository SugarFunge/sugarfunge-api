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
    pub amount: Balance,
    pub to: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountOutput {
    pub from: Account,
    pub to: Account,
    pub amount: Balance,
}

#[cfg(not(feature = "keycloak"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceOutput {
    pub balance: Balance,
}

#[cfg(not(feature = "keycloak"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsOutput {
    pub account: Account,
    pub exists: bool,
}

#[cfg(not(feature = "keycloak"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountOutput {
    pub seed: Seed,
    pub account: Account,
}
