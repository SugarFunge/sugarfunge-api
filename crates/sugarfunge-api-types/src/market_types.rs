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
    pub class_id: u64,
    pub asset_id: u64,
    pub action: AmountOpInput,
    pub amount: i128,
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RatesInput {
    pub rates: Vec<AssetRateInput>,
    pub metadata: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetRate {
    pub class_id: u64,
    pub asset_id: u64,
    pub action: RateAction,
    pub amount: i128,
    pub from: RateAccount,
    pub to: RateAccount,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RateBalance {
    pub rate: AssetRate,
    pub balance: i128,
}

#[derive(Serialize, Deserialize)]
pub struct Rates {
    pub rates: Vec<AssetRate>,
    pub metadata: Vec<u8>,
}

impl Into<AssetRate> for AssetRateInput {
    fn into(self) -> AssetRate {
        AssetRate {
            class_id: self.class_id,
            asset_id: self.asset_id,
            action: self.action.into(),
            amount: self.amount,
            from: self.from.into(),
            to: self.to.into(),
        }
    }
}

impl Into<RateAction> for AmountOpInput {
    fn into(self) -> RateAction {
        match self {
            AmountOpInput::Transfer => RateAction::Transfer,
            AmountOpInput::Mint => RateAction::Mint,
            AmountOpInput::Burn => RateAction::Burn,
            AmountOpInput::HasEqual => RateAction::Has(AmountOp::Equal),
            AmountOpInput::HasLessThan => RateAction::Has(AmountOp::LessThan),
            AmountOpInput::HasLessEqualThan => RateAction::Has(AmountOp::LessEqualThan),
            AmountOpInput::HasGreaterThan => RateAction::Has(AmountOp::GreaterThan),
            AmountOpInput::HasGreaterEqualThan => RateAction::Has(AmountOp::GreaterEqualThan),
        }
    }
}

impl Into<RateAccount> for String {
    fn into(self) -> RateAccount {
        match self.as_str() {
            "Buyer" => RateAccount::Buyer,
            "Market" => RateAccount::Market,
            _ => RateAccount::Account(self),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketInput {
    pub seed: String,
    pub market_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketOutput {
    pub market_id: u64,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateInput {
    pub seed: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub rates: RatesInput,
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateOutput {
    pub market_id: u64,
    pub market_rate_id: u64,
    pub who: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    pub seed: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    pub who: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub amount: u128,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsInput {
    pub seed: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsOutput {
    pub buyer: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub amount: u128,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}
