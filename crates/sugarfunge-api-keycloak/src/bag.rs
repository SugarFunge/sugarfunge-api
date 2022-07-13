use crate::state::*;
use crate::util::*;
use crate::user;
use actix_web::{error, web, HttpResponse};
use actix_web_middleware_keycloak_auth::KeycloakClaims;
use serde_json::json;
use sp_core::{crypto::AccountId32, Pair};
use std::str::FromStr;
use subxt::{sp_runtime::traits::IdentifyAccount, PairSigner};
use sugarfunge_api_keycloak_types::bag::*;
use sugarfunge_api_keycloak_types::config::Config;
use sugarfunge_api_keycloak_types::primitives::*;
use sugarfunge_api_keycloak_types::sugarfunge;
use sugarfunge_api_keycloak_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

pub async fn register(
    data: web::Data<AppState>,
    req: web::Json<RegisterInput>,
    claims: KeycloakClaims<sugarfunge_api_keycloak_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
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
                    Some(event) => Ok(HttpResponse::Ok().json(RegisterOutput {
                        who: event.who.into(),
                        class_id: event.class_id.into(),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::bag::events::Register"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::register"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::register"),
        }))
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

pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
    claims: KeycloakClaims<sugarfunge_api_keycloak_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
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
                    Some(event) => Ok(HttpResponse::Ok().json(CreateOutput {
                        bag: event.bag.into(),
                        class_id: event.class_id.into(),
                        asset_id: event.asset_id.into(),
                        owners: transform_vec_string_to_account(transform_owners_output(event.owners)),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::bag::events::AccountCreated"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::create"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::create"),
        }))
    } 
}

pub async fn sweep(
    data: web::Data<AppState>,
    req: web::Json<SweepInput>,
    claims: KeycloakClaims<sugarfunge_api_keycloak_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let signer = PairSigner::new(pair);
                let bag = sp_core::crypto::AccountId32::try_from(&req.bag).map_err(map_account_err)?;
                let to = sp_core::sr25519::Public::from_str(&pair_account.as_str()).map_err(map_account_err)?;
                let to = sp_core::crypto::AccountId32::from(to);
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
                    Some(event) => Ok(HttpResponse::Ok().json(SweepOutput {
                        bag: event.bag.into(),
                        who: event.who.into(),
                        to: event.to.into(),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::bag::events::Sweep"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::sweep"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::sweep"),
        }))
    } 
}

pub async fn deposit(
    data: web::Data<AppState>,
    req: web::Json<DepositInput>,
    claims: KeycloakClaims<sugarfunge_api_keycloak_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
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
                    Some(event) => Ok(HttpResponse::Ok().json(DepositOutput {
                        bag: event.bag.into(),
                        who: event.who.into(),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::bag::events::Deposit"),
                        description: format!(""),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bag::deposit"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bag::deposit"),
        }))
    } 
}
