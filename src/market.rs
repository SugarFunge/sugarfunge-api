use crate::state::*;
use crate::sugarfunge;
use crate::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use crate::sugarfunge::runtime_types::sugarfunge_market;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::market_types::*;

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
                let account = sp_core::sr25519::Public::from_str(&account).unwrap();
                let account = sp_core::crypto::AccountId32::from(account);
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
                let account = account.to_string();
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
            class_id: self.class_id,
            asset_id: self.asset_id,
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
            class_id: self.class_id,
            asset_id: self.asset_id,
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

fn extrinsinc_rates(
    in_rates: &Vec<AssetRate>,
) -> BoundedVec<sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64>> {
    BoundedVec(
        in_rates
            .iter()
            .map(|rate| {
                <AssetRate as Into<
                    sugarfunge_market::AssetRate<subxt::sp_runtime::AccountId32, u64, u64>,
                >>::into(rate.clone())
            })
            .collect(),
    )
}

fn transform_input(in_rates: &Vec<AssetRateInput>) -> Vec<AssetRate> {
    in_rates
        .iter()
        .map(|rate| <AssetRateInput as Into<AssetRate>>::into(rate.clone()))
        .collect()
}

fn transform_balances(
    in_balances: Vec<sugarfunge_market::RateBalance<subxt::sp_runtime::AccountId32, u64, u64>>,
) -> Vec<RateBalance> {
    in_balances
        .into_iter()
        .map(|rate_balance| RateBalance {
            rate: rate_balance.rate.into(),
            balance: rate_balance.balance,
        })
        .collect()
}

pub async fn create_market(
    data: web::Data<AppState>,
    req: web::Json<CreateMarketInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .market()
        .create_market(req.market_id)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::market::events::Created>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateMarketOutput {
            who: event.who.to_string(),
            market_id: event.market_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Created"),
        })),
    }
}

pub async fn create_market_rate(
    data: web::Data<AppState>,
    req: web::Json<CreateMarketRateInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();

    let rates = transform_input(&req.rates.rates);
    let rates = extrinsinc_rates(&rates);

    let result = api
        .tx()
        .market()
        .create_market_rate(req.market_id, req.market_rate_id, rates)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::market::events::RateCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateMarketRateOutput {
            who: event.who.to_string(),
            market_id: event.market_id,
            market_rate_id: event.market_rate_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::RateCreated"),
        })),
    }
}

pub async fn deposit_assets(
    data: web::Data<AppState>,
    req: web::Json<DepositAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .market()
        .deposit_assets(req.market_id, req.market_rate_id, req.amount)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::market::events::Deposit>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(DepositAssetsOutput {
            who: event.who.to_string(),
            market_id: event.market_id,
            market_rate_id: event.market_rate_id,
            amount: event.amount,
            balances: transform_balances(event.balances),
            success: event.success,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Deposit"),
        })),
    }
}

pub async fn exchange_assets(
    data: web::Data<AppState>,
    req: web::Json<ExchangeAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .market()
        .exchange_assets(req.market_id, req.market_rate_id, req.amount)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::market::events::Exchanged>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ExchangeAssetsOutput {
            buyer: event.buyer.to_string(),
            market_id: event.market_id,
            market_rate_id: event.market_rate_id,
            amount: event.amount,
            balances: transform_balances(event.balances),
            success: event.success,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Exchange"),
        })),
    }
}
