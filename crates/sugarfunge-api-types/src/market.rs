use crate::primitives::*;
use serde::{Deserialize, Serialize};

use crate::sugarfunge::runtime_types::sugarfunge_market;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AmountOp {
    Equal,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
}

impl From<AmountOp> for sugarfunge_market::AmountOp {
    fn from(amount_op: AmountOp) -> Self {
        match amount_op {
            AmountOp::Equal => sugarfunge_market::AmountOp::Equal,
            AmountOp::GreaterEqualThan => sugarfunge_market::AmountOp::GreaterEqualThan,
            AmountOp::GreaterThan => sugarfunge_market::AmountOp::GreaterThan,
            AmountOp::LessEqualThan => sugarfunge_market::AmountOp::LessEqualThan,
            AmountOp::LessThan => sugarfunge_market::AmountOp::LessThan,
        }
    }
}

impl From<sugarfunge_market::AmountOp> for AmountOp {
    fn from(sf_amount_op: sugarfunge_market::AmountOp) -> Self {
        match sf_amount_op {
            sugarfunge_market::AmountOp::Equal => AmountOp::Equal,
            sugarfunge_market::AmountOp::GreaterEqualThan => AmountOp::GreaterEqualThan,
            sugarfunge_market::AmountOp::GreaterThan => AmountOp::GreaterThan,
            sugarfunge_market::AmountOp::LessEqualThan => AmountOp::LessEqualThan,
            sugarfunge_market::AmountOp::LessThan => AmountOp::LessThan,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AMM {
    Constant,
}

impl From<AMM> for sugarfunge_market::AMM {
    fn from(amm: AMM) -> Self {
        match amm {
            AMM::Constant => sugarfunge_market::AMM::Constant,
        }
    }
}

impl From<sugarfunge_market::AMM> for AMM {
    fn from(sf_amm: sugarfunge_market::AMM) -> Self {
        match sf_amm {
            sugarfunge_market::AMM::Constant => AMM::Constant,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAction {
    Transfer(Amount),
    MarketTransfer(AMM, ClassId, AssetId),
    Mint(Amount),
    Burn(Amount),
    Has(AmountOp, Amount),
}

impl From<RateAction> for sugarfunge_market::RateAction<u64, u64> {
    fn from(rate_action: RateAction) -> Self {
        match rate_action {
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

impl From<sugarfunge_market::RateAction<u64, u64>> for RateAction {
    fn from(sf_rate_action: sugarfunge_market::RateAction<u64, u64>) -> Self {
        match sf_rate_action {
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
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RateAccount {
    Market,
    Account(Account),
    Buyer,
}

impl From<RateAccount> for sugarfunge_market::RateAccount<subxt::utils::AccountId32> {
    fn from(rate_account: RateAccount) -> Self {
        match rate_account {
            RateAccount::Buyer => sugarfunge_market::RateAccount::Buyer,
            RateAccount::Market => sugarfunge_market::RateAccount::Market,
            RateAccount::Account(account) => {
                let account = subxt::utils::AccountId32::try_from(&account).unwrap();
                sugarfunge_market::RateAccount::Account(account)
            }
        }
    }
}

impl From<sugarfunge_market::RateAccount<subxt::utils::AccountId32>> for RateAccount {
    fn from(sf_rate_account: sugarfunge_market::RateAccount<subxt::utils::AccountId32>) -> Self {
        match sf_rate_account {
            sugarfunge_market::RateAccount::Buyer => RateAccount::Buyer,
            sugarfunge_market::RateAccount::Market => RateAccount::Market,
            sugarfunge_market::RateAccount::Account(account) => {
                let account = Account::from(account);
                RateAccount::Account(account)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetRate {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub action: RateAction,
    pub from: RateAccount,
    pub to: RateAccount,
}

impl From<AssetRate> for sugarfunge_market::AssetRate<subxt::utils::AccountId32, u64, u64> {
    fn from(asset_rate: AssetRate) -> Self {
        sugarfunge_market::AssetRate::<subxt::utils::AccountId32, u64, u64> {
            class_id: asset_rate.class_id.into(),
            asset_id: asset_rate.asset_id.into(),
            action: asset_rate.action.into(),
            from: asset_rate.from.into(),
            to: asset_rate.to.into(),
        }
    }
}

impl From<sugarfunge_market::AssetRate<subxt::utils::AccountId32, u64, u64>> for AssetRate {
    fn from(
        sf_asset_rate: sugarfunge_market::AssetRate<subxt::utils::AccountId32, u64, u64>,
    ) -> Self {
        AssetRate {
            class_id: sf_asset_rate.class_id.into(),
            asset_id: sf_asset_rate.asset_id.into(),
            action: sf_asset_rate.action.into(),
            from: sf_asset_rate.from.into(),
            to: sf_asset_rate.to.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RateBalance {
    pub rate: AssetRate,
    pub balance: Amount,
}

impl From<sugarfunge_market::RateBalance<subxt::utils::AccountId32, u64, u64>> for RateBalance {
    fn from(
        sf_rate_balance: sugarfunge_market::RateBalance<subxt::utils::AccountId32, u64, u64>,
    ) -> Self {
        RateBalance {
            rate: sf_rate_balance.rate.into(),
            balance: Amount::from(sf_rate_balance.balance),
        }
    }
}

impl From<Account> for RateAccount {
    fn from(account: Account) -> Self {
        match account.as_str() {
            "Buyer" => RateAccount::Buyer,
            "Market" => RateAccount::Market,
            _ => RateAccount::Account(account),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rates {
    pub rates: Vec<AssetRate>,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketInput {
    pub seed: Seed,
    pub market_id: MarketId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketOutput {
    pub market_id: MarketId,
    pub who: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMarketRateInput {
    pub seed: Seed,
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
    pub seed: Seed,
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
    pub seed: Seed,
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
