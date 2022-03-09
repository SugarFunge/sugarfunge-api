use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AmountOp {
    Equal,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AmountOpInput {
    Transfer,
    Mint,
    Burn,
    HasEqual,
    HasLessThan,
    HasLessEqualThan,
    HasGreaterThan,
    HasGreaterEqualThan,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAction {
    Transfer,
    Mint,
    Burn,
    Has(AmountOp),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAccount {
    Market,
    Account(String),
    Buyer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetRateInput {
    class_id: u64,
    asset_id: u64,
    action: AmountOpInput,
    amount: i128,
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RatesInput {
    rates: Vec<AssetRateInput>,
    metadata: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetRate {
    class_id: u64,
    asset_id: u64,
    action: RateAction,
    amount: i128,
    from: RateAccount,
    to: RateAccount,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RateBalance {
    rate: AssetRate,
    balance: i128,
}
#[derive(Serialize, Deserialize)]
pub struct Rates {
    rates: Vec<AssetRate>,
    metadata: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketInput {
    seed: String,
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketOutput {
    market_id: u64,
    who: String,
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateInput {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    rates: RatesInput,
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateOutput {
    market_id: u64,
    market_rate_id: u64,
    who: String,
}
#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    who: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
    balances: Vec<RateBalance>,
    success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsInput {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsOutput {
    buyer: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
    balances: Vec<RateBalance>,
    success: bool,
}
