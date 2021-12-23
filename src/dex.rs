use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

#[derive(Deserialize)]
pub struct CreateDexInput {
    seed: String,
    exchange_id: u32,
    currency_id: u64,
    asset_class_id: u64,
    lp_class_id: u64, // liquidity pool id
}

#[derive(Serialize)]
pub struct CreateDexOutput {
    exchange_id: u32,
    account: String,
}

/// Create dex for currency and asset class
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateDexInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let currency_id = CurrencyId::Id(req.currency_id);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .dex()
        .create_exchange(
            req.exchange_id,
            currency_id,
            req.asset_class_id,
            req.lp_class_id,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::ExchangeCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateDexOutput {
            exchange_id: event.0,
            account: event.1.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    max_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    amounts_in: Vec<u128>,
}

/// Buy assets with currency
pub async fn buy_assets(
    data: web::Data<AppState>,
    req: web::Json<BuyAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .dex()
        .buy_assets(
            req.exchange_id,
            req.asset_ids.clone(),
            req.asset_amounts_out.clone(),
            req.max_currency,
            to,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::CurrencyToAsset>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BuyAssetsOutput {
            exchange_id: event.0,
            who: event.1.to_string(),
            to: event.2.to_string(),
            asset_ids: event.3,
            asset_amounts_out: event.4,
            amounts_in: event.5,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    min_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    amounts_out: Vec<u128>,
}

/// Sell assets for currency
pub async fn sell_assets(
    data: web::Data<AppState>,
    req: web::Json<SellAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .dex()
        .sell_assets(
            req.exchange_id,
            req.asset_ids.clone(),
            req.asset_amounts_in.clone(),
            req.min_currency,
            to,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::AssetToCurrency>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(SellAssetsOutput {
            exchange_id: event.0,
            who: event.1.to_string(),
            to: event.2.to_string(),
            asset_ids: event.3,
            asset_amounts_in: event.4,
            amounts_out: event.5,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    max_currencies: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityOutput {
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}

/// Add liquidity to dex
pub async fn add_liquidity(
    data: web::Data<AppState>,
    req: web::Json<AddLiquidityInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .dex()
        .add_liquidity(
            req.exchange_id,
            to,
            req.asset_ids.clone(),
            req.asset_amounts.clone(),
            req.max_currencies.clone(),
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::LiquidityAdded>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(AddLiquidityOutput {
            who: event.0.to_string(),
            to: event.1.to_string(),
            asset_ids: event.2,
            asset_amounts: event.3,
            currency_amounts: event.4,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    liquidities: Vec<u128>,
    min_currencies: Vec<u128>,
    min_assets: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityOutput {
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}

/// Remove liquidity from dex
pub async fn remove_liquidity(
    data: web::Data<AppState>,
    req: web::Json<RemoveLiquidityInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .dex()
        .remove_liquidity(
            req.exchange_id,
            to,
            req.asset_ids.clone(),
            req.liquidities.clone(),
            req.min_currencies.clone(),
            req.min_assets.clone(),
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::LiquidityRemoved>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveLiquidityOutput {
            who: event.0.to_string(),
            to: event.1.to_string(),
            asset_ids: event.2,
            asset_amounts: event.3,
            currency_amounts: event.4,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}
