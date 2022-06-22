use crate::state::*;
use crate::user;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use actix_web_middleware_keycloak_auth::KeycloakClaims;
use serde_json::json;
use subxt::{PairSigner};
use sugarfunge_api_types::config::Config;
use sugarfunge_api_types::market::*;
use sugarfunge_api_types::primitives::*;
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

fn transform_balances(
    in_balances: Vec<sugarfunge_market::RateBalance<subxt::sp_runtime::AccountId32, u64, u64>>,
) -> Vec<RateBalance> {
    in_balances
        .into_iter()
        .map(|rate_balance| RateBalance {
            rate: rate_balance.rate.into(),
            balance: Amount::from(rate_balance.balance),
        })
        .collect()
}

pub async fn create_market(
    data: web::Data<AppState>,
    req: web::Json<CreateMarketInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let signer = PairSigner::new(pair);
                let api = &data.api;
                let result = api
                    .tx()
                    .market()
                    .create_market(req.market_id.into())
                    .map_err(map_subxt_err)?
                    .sign_and_submit_then_watch(&signer, Default::default())
                    .await
                    .map_err(map_subxt_err)?
                    .wait_for_finalized_success()
                    .await
                    .map_err(map_sf_err)?;
                let result = result
                    .find_first::<sugarfunge::market::events::Created>()
                    .map_err(map_subxt_err)?;
                match result {
                    Some(event) => Ok(HttpResponse::Ok().json(CreateMarketOutput {
                        who: event.who.into(),
                        market_id: event.market_id.into(),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::market::events::Created"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in market::create_market"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in market::create_market"),
        }))
    }  
}

pub async fn create_market_rate(
    data: web::Data<AppState>,
    req: web::Json<CreateMarketRateInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let signer = PairSigner::new(pair);
                let api = &data.api;
                let rates = &req.rates.rates; //transform_input(&req.rates.rates);
                let rates = extrinsinc_rates(&rates);
                let result = api
                    .tx()
                    .market()
                    .create_market_rate(req.market_id.into(), u64::from(req.market_rate_id), rates)
                    .map_err(map_subxt_err)?
                    .sign_and_submit_then_watch(&signer, Default::default())
                    .await
                    .map_err(map_subxt_err)?
                    .wait_for_finalized_success()
                    .await
                    .map_err(map_sf_err)?;
                let result = result
                    .find_first::<sugarfunge::market::events::RateCreated>()
                    .map_err(map_subxt_err)?;
                match result {
                    Some(event) => Ok(HttpResponse::Ok().json(CreateMarketRateOutput {
                        who: event.who.into(),
                        market_id: event.market_id.into(),
                        market_rate_id: MarketId::from(event.market_rate_id),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::market::events::RateCreated"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in market::create_market_rate"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in market::create_market_rate"),
        }))
    }       
}

pub async fn deposit_assets(
    data: web::Data<AppState>,
    req: web::Json<DepositAssetsInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let signer = PairSigner::new(pair);
                let api = &data.api;
                let result = api
                    .tx()
                    .market()
                    .deposit(
                        req.market_id.into(),
                        u64::from(req.market_rate_id),
                        req.amount.into(),
                    )
                    .map_err(map_subxt_err)?
                    .sign_and_submit_then_watch(&signer, Default::default())
                    .await
                    .map_err(map_subxt_err)?
                    .wait_for_finalized_success()
                    .await
                    .map_err(map_sf_err)?;
                let result = result
                    .find_first::<sugarfunge::market::events::Deposit>()
                    .map_err(map_subxt_err)?;
                match result {
                    Some(event) => Ok(HttpResponse::Ok().json(DepositAssetsOutput {
                        who: event.who.into(),
                        market_id: event.market_id.into(),
                        market_rate_id: MarketId::from(event.market_rate_id),
                        amount: event.amount.into(),
                        balances: transform_balances(event.balances),
                        success: event.success,
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::market::events::Deposit"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in market::deposit_assets"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in market::deposit_assets"),
        }))
    }  
}

pub async fn exchange_assets(
    data: web::Data<AppState>,
    req: web::Json<ExchangeAssetsInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let signer = PairSigner::new(pair);
                let api = &data.api;
                let result = api
                    .tx()
                    .market()
                    .exchange_assets(
                        req.market_id.into(),
                        u64::from(req.market_rate_id),
                        req.amount.into(),
                    )
                    .map_err(map_subxt_err)?
                    .sign_and_submit_then_watch(&signer, Default::default())
                    .await
                    .map_err(map_subxt_err)?
                    .wait_for_finalized_success()
                    .await
                    .map_err(map_sf_err)?;
                let result = result
                    .find_first::<sugarfunge::market::events::Exchanged>()
                    .map_err(map_subxt_err)?;
                match result {
                    Some(event) => Ok(HttpResponse::Ok().json(ExchangeAssetsOutput {
                        buyer: event.buyer.into(),
                        market_id: event.market_id.into(),
                        market_rate_id: MarketId::from(event.market_rate_id),
                        amount: event.amount.into(),
                        balances: transform_balances(event.balances),
                        success: event.success,
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::market::events::Exchange"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in market::exchange_assets"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in market::exchange_assets"),
        }))
    }         
}
