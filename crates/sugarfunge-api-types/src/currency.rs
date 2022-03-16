use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyOutput {
    pub currency: Currency,
    pub who: Account,
    pub amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceInput {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceOutput {
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyInput {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyOutput {
    pub total_supply: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyOutput {
    pub currency: Currency,
    pub amount: Balance,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyInput {
    pub seed: Seed,
    pub currency: Currency,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyOutput {
    pub currency: Currency,
    pub amount: Balance,
    pub who: Account,
}
