use std::str::FromStr;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use sp_core::crypto::AccountId32;
use subxt::PairSigner;
use sugarfunge_api_types::bag::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

#[cfg(feature = "keycloak")]
use crate::config::Config;
#[cfg(feature = "keycloak")]
use crate::user;
#[cfg(feature = "keycloak")]
use actix_web_middleware_keycloak_auth::KeycloakClaims;
#[cfg(feature = "keycloak")]
use sp_core::Pair;
#[cfg(feature = "keycloak")]
use subxt::sp_runtime::traits::IdentifyAccount;

/// Registers a class for the bag
#[cfg(not(feature = "keycloak"))]
pub async fn register(
    data: web::Data<AppState>,
    req: web::Json<RegisterInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match bag_register_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to register bag"),
            description: format!("Error in bag::register"),
        })),
    }    
}

/// Registers a class for the bag
#[cfg(feature = "keycloak")]
pub async fn register(
    data: web::Data<AppState>,
    req: web::Json<RegisterInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match bag_register_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to register bag"),
                        description: format!("Error in bag::register"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::register"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::register"),
        })),
    }
}

pub async fn bag_register_call(
    data: web::Data<AppState>,
    req: web::Json<RegisterInput>,
    seed: Seed,
) -> error::Result<web::Json<RegisterOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let result = api
        .tx()
        .bag()
        .register(req.class_id.into(), metadata)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bag::events::Register>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(RegisterOutput {
            who: event.who.into(),
            class_id: event.class_id.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bag::events::Register"),
            description: format!(""),
        })),
    }
}

pub fn transform_owners_input(in_owners: Vec<String>) -> Vec<AccountId32> {
    in_owners
        .into_iter()
        .map(|current_owner| {
            sp_core::crypto::AccountId32::from(
                sp_core::sr25519::Public::from_str(&current_owner).unwrap(),
            )
        })
        .collect()
}

pub fn transform_owners_output(in_owners: Vec<AccountId32>) -> Vec<String> {
    in_owners
        .into_iter()
        .map(|current_owner| current_owner.to_string())
        .collect()
}

/// Creates a bag
#[cfg(not(feature = "keycloak"))]
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match bag_create_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to create bag"),
            description: format!("Error in bag::create"),
        })),
    }
}

/// Creates a bag
#[cfg(feature = "keycloak")]
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match bag_create_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to create bag"),
                        description: format!("Error in bag::create"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::create"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::create"),
        })),
    }
}

pub async fn bag_create_call(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
    seed: Seed,
) -> error::Result<web::Json<CreateOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let owners = transform_owners_input(transform_vec_account_to_string(req.owners.clone()));
    let api = &data.api;
    let result = api
        .tx()
        .bag()
        .create(
            req.class_id.into(),
            owners,
            transform_vec_balance_to_u128(&req.shares),
        )
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bag::events::Created>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(CreateOutput {
            bag: event.bag.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            owners: transform_vec_string_to_account(transform_owners_output(event.owners)),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bag::events::AccountCreated"),
            description: format!(""),
        })),
    }
}

/// Sweeps the content of the bag
#[cfg(not(feature = "keycloak"))]
pub async fn sweep(
    data: web::Data<AppState>,
    req: web::Json<SweepInput>,
) -> error::Result<HttpResponse> {    
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let user_seed = Seed::from(req.seed.clone());
    match bag_sweep_call(data, req, user_seed, to).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to sweep bag"),
            description: format!("Error in bag::sweep"),
        })),
    }
}

/// Sweeps the content of the bag
#[cfg(feature = "keycloak")]
pub async fn sweep(
    data: web::Data<AppState>,
    req: web::Json<SweepInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {    
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account = sp_core::sr25519::Public::from_str(&pair_account.as_str()).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                match bag_sweep_call(data, req, user_seed, account).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to sweep bag"),
                        description: format!("Error in bag::sweep"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::sweep"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::sweep"),
        })),
    }
}

pub async fn bag_sweep_call(
    data: web::Data<AppState>,
    req: web::Json<SweepInput>,
    seed: Seed,
    to: AccountId32
) -> error::Result<web::Json<SweepOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let bag = sp_core::crypto::AccountId32::try_from(&req.bag).map_err(map_account_err)?;
    let api = &data.api;
    let result = api
        .tx()
        .bag()
        .sweep(to, bag)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bag::events::Sweep>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(SweepOutput {
            bag: event.bag.into(),
            who: event.who.into(),
            to: event.to.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bag::events::Sweep"),
            description: format!(""),
        })),
    }
}

/// Deposits content of the bag
#[cfg(not(feature = "keycloak"))]
pub async fn deposit(
    data: web::Data<AppState>,
    req: web::Json<DepositInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match bag_deposit_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to sweep bag"),
            description: format!("Error in bag::sweep"),
        })),
    }
}

/// Deposits content of the bag
#[cfg(feature = "keycloak")]
pub async fn deposit(
    data: web::Data<AppState>,
    req: web::Json<DepositInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match bag_deposit_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to sweep bag"),
                        description: format!("Error in bag::sweep"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::deposit"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::deposit"),
        })),
    }
}

pub async fn bag_deposit_call(
    data: web::Data<AppState>,
    req: web::Json<DepositInput>,
    seed: Seed,
) -> error::Result<web::Json<DepositOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let bag = sp_core::crypto::AccountId32::try_from(&req.bag).map_err(map_account_err)?;
    let api = &data.api;
    let result = api
        .tx()
        .bag()
        .deposit(
            bag,
            transform_vec_classid_to_u64(req.class_ids.clone()),
            transform_doublevec_assetid_to_u64(req.asset_ids.clone()),
            transform_doublevec_balance_to_u128(req.amounts.clone()),
        )
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bag::events::Deposit>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(DepositOutput {
            bag: event.bag.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bag::events::Deposit"),
            description: format!(""),
        })),
    }
}
