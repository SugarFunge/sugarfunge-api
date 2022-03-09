use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Currency {
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyInput {
    seed: String,
    currency: Currency,
    amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCurrencyOutput {
    currency: Currency,
    who: String,
    amount: i128,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceInput {
    currency: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceOutput {
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyInput {
    currency: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyOutput {
    total_supply: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyInput {
    seed: String,
    currency: Currency,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyOutput {
    currency: Currency,
    amount: u128,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyInput {
    seed: String,
    currency: Currency,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnCurrencyOutput {
    currency: Currency,
    amount: u128,
    who: String,
}
