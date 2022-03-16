use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::PairSigner;
use sugarfunge_api_types::market::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use sugarfunge_api_types::sugarfunge::runtime_types::sugarfunge_market;

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
        .create_market(req.market_id.into())
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
            who: event.who.into(),
            market_id: event.market_id.into(),
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
        .create_market_rate(req.market_id.into(), req.market_rate_id, rates)
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
            who: event.who.into(),
            market_id: event.market_id.into(),
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
        .deposit_assets(req.market_id.into(), req.market_rate_id, req.amount.into())
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
            who: event.who.into(),
            market_id: event.market_id.into(),
            market_rate_id: event.market_rate_id,
            amount: event.amount.into(),
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
        .exchange_assets(req.market_id.into(), req.market_rate_id, req.amount.into())
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
            buyer: event.buyer.into(),
            market_id: event.market_id.into(),
            market_rate_id: event.market_rate_id,
            amount: event.amount.into(),
            balances: transform_balances(event.balances),
            success: event.success,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::market::events::Exchange"),
        })),
    }
}
