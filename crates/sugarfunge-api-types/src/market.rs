use crate::primitives::*;
use serde::{Deserialize, Serialize};

use crate::sugarfunge::runtime_types::sugarfunge_market;

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
    Account(Account),
    Buyer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetRateInput {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub action: AmountOpInput,
    pub amount: i128,
    pub from: Account,
    pub to: Account,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RatesInput {
    pub rates: Vec<AssetRateInput>,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetRate {
    pub class_id: ClassId,
    pub asset_id: AssetId,
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
    pub metadata: serde_json::Value,
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

impl Into<sugarfunge_market::AmountOp> for AmountOp {
    fn into(self) -> sugarfunge_market::AmountOp {
        match self {
            AmountOp::Equal => sugarfunge_market::AmountOp::Equal,
            AmountOp::GreaterEqualThan => sugarfunge_market::AmountOp::GreaterEqualThan,
            AmountOp::GreaterThan => sugarfunge_market::AmountOp::GreaterThan,
            AmountOp::LessEqualThan => sugarfunge_market::AmountOp::LessEqualThan,
            AmountOp::LessThan => sugarfunge_market::AmountOp::LessThan,
        }
    }
}

impl Into<AmountOp> for sugarfunge_market::AmountOp {
    fn into(self) -> AmountOp {
        match self {
            sugarfunge_market::AmountOp::Equal => AmountOp::Equal,
            sugarfunge_market::AmountOp::GreaterEqualThan => AmountOp::GreaterEqualThan,
            sugarfunge_market::AmountOp::GreaterThan => AmountOp::GreaterThan,
            sugarfunge_market::AmountOp::LessEqualThan => AmountOp::LessEqualThan,
            sugarfunge_market::AmountOp::LessThan => AmountOp::LessThan,
        }
    }
}

impl Into<sugarfunge_market::RateAccount<subxt::sp_runtime::AccountId32>> for RateAccount {
    fn into(self) -> sugarfunge_market::RateAccount<subxt::sp_runtime::AccountId32> {
        match self {
            RateAccount::Buyer => sugarfunge_market::RateAccount::Buyer,
            RateAccount::Market => sugarfunge_market::RateAccount::Market,
            RateAccount::Account(account) => {
                let account = sp_core::crypto::AccountId32::try_from(&account).unwrap();
                sugarfunge_market::RateAccount::Account(account)
            }
        }
    }
}

impl Into<RateAccount> for sugarfunge_market::RateAccount<subxt::sp_runtime::AccountId32> {
    fn into(self) -> RateAccount {
        match self {
            sugarfunge_market::RateAccount::Buyer => RateAccount::Buyer,
            sugarfunge_market::RateAccount::Market => RateAccount::Market,
            sugarfunge_market::RateAccount::Account(account) => {
                let account = Account::from(account);
                RateAccount::Account(account)
            }
        }
    }
}

impl Into<sugarfunge_market::RateAction> for RateAction {
    fn into(self) -> sugarfunge_market::RateAction {
        match self {
            RateAction::Transfer => sugarfunge_market::RateAction::Transfer,
            RateAction::Mint => sugarfunge_market::RateAction::Mint,
            RateAction::Burn => sugarfunge_market::RateAction::Burn,
            RateAction::Has(op) => sugarfunge_market::RateAction::Has(op.into()),
        }
    }
}

impl Into<RateAction> for sugarfunge_market::RateAction {
    fn into(self) -> RateAction {
        match self {
            sugarfunge_market::RateAction::Transfer => RateAction::Transfer,
            sugarfunge_market::RateAction::Mint => RateAction::Mint,
            sugarfunge_market::RateAction::Burn => RateAction::Burn,
            sugarfunge_market::RateAction::Has(op) => RateAction::Has(op.into()),
        }
    }
}

impl Into<sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64>> for AssetRate {
    fn into(self) -> sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64> {
        sugarfunge_market::AssetRate::<subxt::sp_runtime::AccountId32, u64, u64> {
            class_id: self.class_id.into(),
            asset_id: self.asset_id.into(),
            action: self.action.into(),
            amount: self.amount,
            from: self.from.into(),
            to: self.to.into(),
            __subxt_unused_type_params: Default::default(),
        }
    }
}

impl Into<AssetRate> for sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64> {
    fn into(self) -> AssetRate {
        AssetRate {
            class_id: self.class_id.into(),
            asset_id: self.asset_id.into(),
            action: self.action.into(),
            amount: self.amount,
            from: self.from.into(),
            to: self.to.into(),
        }
    }
}

impl Into<RateBalance>
    for sugarfunge_market::RateBalance<subxt::sp_runtime::AccountId32, u64, u64>
{
    fn into(self) -> RateBalance {
        RateBalance {
            rate: self.rate.into(),
            balance: self.balance,
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

impl Into<RateAccount> for Account {
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
    pub seed: Seed,
    pub market_id: MarketId,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketOutput {
    pub market_id: MarketId,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateInput {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub rates: RatesInput,
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateOutput {
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub who: Account,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    pub who: Account,
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub amount: Balance,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsInput {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAssetsOutput {
    pub buyer: Account,
    pub market_id: MarketId,
    pub market_rate_id: u64,
    pub amount: Balance,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}
