use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: i128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCurrencyOutput {
    pub currency: Currency,
    pub who: Account,
    pub amount: i128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyIssuanceInput {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyIssuanceOutput {
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencySupplyInput {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencySupplyOutput {
    pub total_supply: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MintCurrencyOutput {
    pub currency: Currency,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnCurrencyOutput {
    pub currency: Currency,
    pub amount: Balance,
    pub who: Account,
}
