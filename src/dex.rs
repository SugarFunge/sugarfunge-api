use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::PairSigner;
use sugarfunge_api_types::dex::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

/// Create dex for currency and asset class
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateDexInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
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
            exchange_id: event.exchange_id,
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
        })),
    }
}

/// Buy assets with currency
pub async fn buy_assets(
    data: web::Data<AppState>,
    req: web::Json<BuyAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
            exchange_id: event.exchange_id,
            who: event.who.into(),
            to: event.to.into(),
            asset_ids: event.asset_ids,
            asset_amounts_out: event.asset_amounts_out,
            currency_amounts_in: event.currency_amounts_in,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

/// Sell assets for currency
pub async fn sell_assets(
    data: web::Data<AppState>,
    req: web::Json<SellAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
            exchange_id: event.exchange_id,
            who: event.who.into(),
            to: event.to.into(),
            asset_ids: event.asset_ids,
            asset_amounts_in: event.asset_amounts_in,
            currency_amounts_out: event.currency_amounts_out,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

/// Add liquidity to dex
pub async fn add_liquidity(
    data: web::Data<AppState>,
    req: web::Json<AddLiquidityInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
            exchange_id: event.exchange_id,
            who: event.who.into(),
            to: event.to.into(),
            asset_ids: event.asset_ids,
            asset_amounts: event.asset_amounts,
            currency_amounts: event.currency_amounts,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}

/// Remove liquidity from dex
pub async fn remove_liquidity(
    data: web::Data<AppState>,
    req: web::Json<RemoveLiquidityInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
            exchange_id: event.exchange_id,
            who: event.who.into(),
            to: event.to.into(),
            asset_ids: event.asset_ids,
            asset_amounts: event.asset_amounts,
            currency_amounts: event.currency_amounts,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::dex::events::CurrencyToAsset"),
        })),
    }
}
