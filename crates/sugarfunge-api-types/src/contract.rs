use serde::{Deserialize, Serialize};

use crate::primitives::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractTransactionInput {
    pub account_address: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractTotalSupplyOutput {
    pub total_supply: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractAllowanceInput {
    pub owner_address: String,
    pub spender_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractAllowanceOutput {
    pub allowance: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertFulaInput {
    pub seed: Seed,
    pub wallet_account: String,
    pub amount: Balance,
}
