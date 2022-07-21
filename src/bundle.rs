use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Encode;
use hex::ToHex;
use serde_json::json;
use sp_core::crypto::AccountId32;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::bundle::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

#[cfg(feature = "keycloak")]
use sugarfunge_api_types::config::Config;
#[cfg(feature = "keycloak")]
use crate::user;
#[cfg(feature = "keycloak")]
use actix_web_middleware_keycloak_auth::KeycloakClaims;
#[cfg(feature = "keycloak")]
use sp_core::Pair;
#[cfg(feature = "keycloak")]
use subxt::sp_runtime::traits::IdentifyAccount;

fn hash(s: &[u8]) -> sp_core::H256 {
    sp_io::hashing::blake2_256(s).into()
}

/// Registers a bundle
#[cfg(not(feature = "keycloak"))]
pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match register_bundle_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to register bundle"),
            description: format!("Error in bundle::register"),
        })),
    }
}

/// Registers a bundle
#[cfg(feature = "keycloak")]
pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match register_bundle_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to register bundle"),
                        description: format!("Error in bundle::register"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bundle::register_bundle"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bundle::register_bundle"),
        })),
    }    
}

pub async fn register_bundle_call(
        data: web::Data<AppState>,
        req: web::Json<RegisterBundleInput>,
        seed: Seed,
) -> error::Result<web::Json<RegisterBundleOutput>, HttpResponse> {
    let schema = (
        BoundedVec(transform_vec_classid_to_u64(req.schema.class_ids.to_vec())),
        BoundedVec(
            req.schema
                .asset_ids
                .iter()
                .map(|x| BoundedVec(transform_vec_assetid_to_u64(x.to_vec())))
                .collect(),
        ),
        BoundedVec(
            req.schema
                .amounts
                .iter()
                .map(|x| BoundedVec(transform_vec_balance_to_u128(&x.to_vec())))
                .collect(),
        ),
    );
    let bundle_id = hash(&schema.encode());
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let result = api
        .tx()
        .bundle()
        .register_bundle(
            req.class_id.into(),
            req.asset_id.into(),
            bundle_id,
            schema,
            metadata,
        )
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bundle::events::Register>()
        .map_err(map_subxt_err)?;    
    match result {
        Some(event) => Ok(web::Json(RegisterBundleOutput {
            who: event.who.into(),
            bundle_id: event.bundle_id.encode_hex(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
            description: format!(""),
        })),
    }
}

/// Mints a bundle
#[cfg(not(feature = "keycloak"))]
pub async fn mint_bundle(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
) -> error::Result<HttpResponse> {
    let account_from = sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let user_seed = Seed::from(req.seed.clone());
    match mint_bundle_call(data, req, user_seed, account_from, account_to).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to mint bundle"),
            description: format!("Error in bundle::mint"),
        })),
    }    
}

/// Mints a bundle
#[cfg(feature = "keycloak")]
pub async fn mint_bundle(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account_from = sp_core::sr25519::Public::from_str(&pair_account.as_str()).map_err(map_account_err)?;
                let account_from = sp_core::crypto::AccountId32::from(account_from);
                let account_to = account_from.clone();
                match mint_bundle_call(data, req, user_seed, account_from, account_to).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to mint bundle"),
                        description: format!("Error in bundle::mint"),
                    })),
                }  
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bundle::mint_bundle"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bundle::mint_bundle"),
        })),
    }      
}

pub async fn mint_bundle_call(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
    seed: Seed,
    account_from: AccountId32,
    account_to: AccountId32,
) -> error::Result<web::Json<MintBundleOutput>, HttpResponse> {
    let bundle_id = sp_core::H256::from_str(&req.bundle_id.as_str()).unwrap_or_default();
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let result = api
        .tx()
        .bundle()
        .mint_bundle(account_from, account_to, bundle_id, req.amount.into())
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bundle::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(MintBundleOutput {
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.encode_hex(),
            amount: event.amount.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Mint"),
            description: format!(""),
        })),
    }
}

/// Burns a bundle
#[cfg(not(feature = "keycloak"))]
pub async fn burn_bundle(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
) -> error::Result<HttpResponse> {
    let account_from = sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let user_seed = Seed::from(req.seed.clone());
    match burn_bundle_call(data, req, user_seed, account_from, account_to).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to burn bundle"),
            description: format!("Error in bundle::burn"),
        })),
    }    
}

/// Burns a bundle
#[cfg(feature = "keycloak")]
pub async fn burn_bundle(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account_from = sp_core::sr25519::Public::from_str(&pair_account.as_str()).map_err(map_account_err)?;
                let account_from = sp_core::crypto::AccountId32::from(account_from);
                let account_to = account_from.clone();
                match burn_bundle_call(data, req, user_seed, account_from, account_to).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to burn bundle"),
                        description: format!("Error in bundle::burn"),
                    })),
                }  
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in bundle::burn_bundle"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in bundle::burn_bundle"),
        })),
    }      
}

pub async fn burn_bundle_call(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
    seed: Seed,
    account_from: AccountId32,
    account_to: AccountId32,
) -> error::Result<web::Json<BurnBundleOutput>, HttpResponse> {
    let bundle_id = sp_core::H256::from_str(&req.bundle_id.as_str()).unwrap_or_default();
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let result = api
        .tx()
        .bundle()
        .burn_bundle(account_from, account_to, bundle_id, req.amount.into())
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::bundle::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(BurnBundleOutput {
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.encode_hex(),
            amount: event.amount.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Burn"),
            description: format!(""),
        })),
    }
}
