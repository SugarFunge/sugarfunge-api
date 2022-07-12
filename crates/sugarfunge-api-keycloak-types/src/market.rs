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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AMM {
    Constant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAction {
    Transfer(Amount),
    MarketTransfer(AMM, ClassId, AssetId),
    Mint(Amount),
    Burn(Amount),
    Has(AmountOp, Amount),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAccount {
    Market,
    Account(Account),
    Buyer,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetRate {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub action: RateAction,
    pub from: RateAccount,
    pub to: RateAccount,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RateBalance {
    pub rate: AssetRate,
    pub balance: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rates {
    pub rates: Vec<AssetRate>,
    pub metadata: serde_json::Value,
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

impl Into<sugarfunge_market::AMM> for AMM {
    fn into(self) -> sugarfunge_market::AMM {
        match self {
            AMM::Constant => sugarfunge_market::AMM::Constant,
        }
    }
}

impl Into<AMM> for sugarfunge_market::AMM {
    fn into(self) -> AMM {
        match self {
            sugarfunge_market::AMM::Constant => AMM::Constant,
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

impl Into<sugarfunge_market::RateAction<u64, u64>> for RateAction {
    fn into(self) -> sugarfunge_market::RateAction<u64, u64> {
        match self {
            RateAction::Transfer(amount) => {
                sugarfunge_market::RateAction::Transfer(i128::from(amount))
            }
            RateAction::Mint(amount) => sugarfunge_market::RateAction::Mint(i128::from(amount)),
            RateAction::Burn(amount) => sugarfunge_market::RateAction::Burn(i128::from(amount)),
            RateAction::Has(op, amount) => {
                sugarfunge_market::RateAction::Has(op.into(), i128::from(amount))
            }
            RateAction::MarketTransfer(amm, class_id, asset_id) => {
                sugarfunge_market::RateAction::MarketTransfer(
                    amm.into(),
                    class_id.into(),
                    asset_id.into(),
                )
            }
        }
    }
}

impl Into<RateAction> for sugarfunge_market::RateAction<u64, u64> {
    fn into(self) -> RateAction {
        match self {
            sugarfunge_market::RateAction::Transfer(amount) => {
                RateAction::Transfer(Amount::from(amount))
            }
            sugarfunge_market::RateAction::Mint(amount) => RateAction::Mint(Amount::from(amount)),
            sugarfunge_market::RateAction::Burn(amount) => RateAction::Burn(Amount::from(amount)),
            sugarfunge_market::RateAction::Has(op, amount) => {
                RateAction::Has(op.into(), Amount::from(amount))
            }
            sugarfunge_market::RateAction::MarketTransfer(amm, class_id, asset_id) => {
                RateAction::MarketTransfer(amm.into(), class_id.into(), asset_id.into())
            }
            sugarfunge_market::RateAction::__Ignore(_) => RateAction::Transfer(Amount::from(0)),
        }
    }
}

impl Into<sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64>> for AssetRate {
    fn into(self) -> sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64> {
        sugarfunge_market::AssetRate::<subxt::sp_runtime::AccountId32, u64, u64> {
            class_id: self.class_id.into(),
            asset_id: self.asset_id.into(),
            action: self.action.into(),
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
            balance: Amount::from(self.balance),
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
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketInput {
    pub market_id: MarketId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketOutput {
    pub market_id: MarketId,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketRateInput {
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub rates: Rates,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketRateOutput {
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositAssetsInput {
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositAssetsOutput {
    pub who: Account,
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: Balance,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeAssetsInput {
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeAssetsOutput {
    pub buyer: Account,
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: Balance,
    pub balances: Vec<RateBalance>,
    pub success: bool,
}
