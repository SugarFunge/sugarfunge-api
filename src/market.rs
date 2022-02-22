use std::collections::BTreeMap;

use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub enum AmountOp {
    Equal,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
}

#[derive(Serialize, Deserialize)]
pub enum RateAction {
    Transfer,
    Mint,
    Burn,
    Has(AmountOp),
}

#[derive(Serialize, Deserialize)]
pub enum RateAccount {
    Market,
    Account(String),
    Buyer,
}

#[derive(Serialize, Deserialize)]
pub struct AssetRate {
    class_id: u64,
    asset_id: u64,
    action: RateAction,
    amount: u128,
    from: RateAccount,
    to: RateAccount,
}

#[derive(Serialize, Deserialize)]
pub struct MarketRate {
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

#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateInput {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    market_rate: MarketRate,
}
#[derive(Serialize, Deserialize)]
pub struct CreateMarketRateOutput {
    market_id: u64,
    market_rate_id: u64,
    who: String,
}

pub async fn create_market_rate(
    data: web::Data<AppState>,
    req: web::Json<CreateMarketRateInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .market()
        .create_market_rate(req.market_id, req.market_rate_id, req.market_rate)
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
    balances: BTreeMap<AssetRate, i128>,
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
            balances: event.balances,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Deposit"),
        })),
    }
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
    balances: BTreeMap<AssetRate, i128>,
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
            balances: event.balances,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Exchange"),
        })),
    }
}
