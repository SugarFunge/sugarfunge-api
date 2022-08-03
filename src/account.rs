use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpRequest, HttpResponse};
use rand::prelude::*;
use serde_json::json;
use sp_core::crypto::AccountId32;
use sp_core::Pair;
use subxt::{sp_runtime::traits::IdentifyAccount, PairSigner};
use sugarfunge_api_types::account::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;

#[cfg(feature = "keycloak")]
use crate::config::Config;
#[cfg(feature = "keycloak")]
use crate::user;
#[cfg(feature = "keycloak")]
use actix_web_middleware_keycloak_auth::KeycloakClaims;
#[cfg(feature = "keycloak")]
use std::str::FromStr;

/// Generate a unique seed and its associated account
pub async fn create(_req: HttpRequest) -> error::Result<HttpResponse> {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let seed = hex::encode(seed);
    let seed = format!("//{}", seed);
    let seed = Seed::from(seed);
    let pair = get_pair_from_seed(&seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(CreateAccountOutput {
        seed,
        account: Account::from(format!("{}", account)),
    }))
}

/// Compute account from seed
#[cfg(not(feature = "keycloak"))]
pub async fn seeded(req: web::Json<SeededAccountInput>) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match account_seeded_call(user_seed).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
    }
}

/// Compute account from seed
#[cfg(feature = "keycloak")]
pub async fn seeded(
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match account_seeded_call(user_seed).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
                    description: format!("Error in account::seeded"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::seeded"),
        })),
    }
}

async fn account_seeded_call(seed: Seed) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(SeededAccountOutput {
        seed: seed.clone(),
        account: Account::from(format!("{}", account)),
    }))
}

/// Fund a given account with amount
pub async fn fund(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    match account_fund_call(data, req, account).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
    }
}

async fn account_fund_call(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
    to: AccountId32,
) -> error::Result<HttpResponse, actix_web::Error> {
    let account = subxt::sp_runtime::MultiAddress::Id(to);
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let amount_input = req.amount;
    let api = &data.api;
    let result = api
        .tx()
        .balances()
        .transfer(account, amount_input.into())
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::balances::events::Transfer>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(FundAccountOutput {
            from: event.from.into(),
            to: event.to.into(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
            description: format!("Error in account::fund"),
        })),
    }
}

/// Get balance for given account
#[cfg(not(feature = "keycloak"))]
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AccountBalanceInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::crypto::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    match account_balance_call(data, account).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
    }
}

/// Get balance for given account
#[cfg(feature = "keycloak")]
pub async fn balance(
    data: web::Data<AppState>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                match account_balance_call(data, account).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
                    description: format!("Error in account::balance"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::balance"),
        })),
    }
}

async fn account_balance_call(
    data: web::Data<AppState>,
    account: AccountId32,
) -> error::Result<HttpResponse, actix_web::Error> {
    let api = &data.api;
    let result = api.storage().system().account(&account, None).await;
    let data = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AccountBalanceOutput {
        balance: data.data.free.into(),
    }))
}

/// Check if account exists and is active
#[cfg(not(feature = "keycloak"))]
pub async fn exists(
    data: web::Data<AppState>,
    req: web::Json<AccountExistsInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::crypto::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    match account_exists_call(data, account).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
    }
}

/// Check if account exists and is active
#[cfg(feature = "keycloak")]
pub async fn exists(
    data: web::Data<AppState>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                match account_exists_call(data, account).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
                    description: format!("Error in account::exists"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::exists"),
        })),
    }
}

async fn account_exists_call(
    data: web::Data<AppState>,
    account: AccountId32,
) -> error::Result<HttpResponse, actix_web::Error> {
    let account_out = account.clone();
    let api = &data.api;
    let result = api.storage().system().account(&account, None).await;
    let data = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AccountExistsOutput {
        account: account_out.into(),
        exists: data.providers > 0,
    }))
}
