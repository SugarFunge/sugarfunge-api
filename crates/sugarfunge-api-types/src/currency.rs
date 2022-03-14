use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub class_id: u64,
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyInput {
    pub seed: String,
    pub currency: Currency,
    pub amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyOutput {
    pub currency: Currency,
    pub who: String,
    pub amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceInput {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceOutput {
    pub amount: u128,
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
    pub seed: String,
    pub currency: Currency,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyOutput {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyInput {
    pub seed: String,
    pub currency: Currency,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyOutput {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}
