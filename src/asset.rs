use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use sp_core::crypto::AccountId32;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::asset::*;
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

#[cfg(not(feature = "keycloak"))]
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    let to = sp_core::sr25519::Public::from_str(&req.owner.as_str()).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    match create_class_call(data, req, user_seed, to).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to create class"),
            description: format!("Error in asset::createClass"),
        })),
    }
}

// Create an asset class for an account
#[cfg(feature = "keycloak")]
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let to = sp_core::sr25519::Public::from_str(&pair_account.as_str())
                    .map_err(map_account_err)?;
                let to = sp_core::crypto::AccountId32::from(to);
                match create_class_call(data, req, user_seed, to).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to create class"),
                        description: format!("Error in asset::createClass"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::create_class"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::create_class"),
        })),
    }
}

async fn create_class_call(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
    seed: Seed,
    to: AccountId32,
) -> error::Result<web::Json<CreateClassOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .create_class(to, req.class_id.into(), metadata)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::ClassCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(CreateClassOutput {
            class_id: event.class_id.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
            description: format!(""),
        })),
    }
}

/// Get class info
pub async fn class_info(
    data: web::Data<AppState>,
    req: web::Json<ClassInfoInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let result = api
        .storage()
        .asset()
        .classes(&req.class_id.into(), None)
        .await;
    let info = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(ClassInfoOutput {
        info: match info {
            Some(info) => Some(ClassInfo {
                class_id: req.class_id.clone(),
                owner: info.owner.into(),
                metadata: serde_json::from_slice(info.metadata.0.as_slice()).unwrap_or_default(),
            }),
            None => None,
        },
    }))
}

/// Create an asset for class
#[cfg(not(feature = "keycloak"))]
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match create_asset_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to create asset"),
            description: format!("Error in asset::create"),
        })),
    }    
}

/// Create an asset for class
#[cfg(feature = "keycloak")]
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match create_asset_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to create asset"),
                        description: format!("Error in asset::create"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::create"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::create"),
        })),
    }  
}

async fn create_asset_call(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
    seed: Seed,
) -> error::Result<web::Json<CreateOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .create_asset(req.class_id.into(), req.asset_id.into(), metadata)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::AssetCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(CreateOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
            description: format!(""),
        })),
    }
}

/// Get asset info
pub async fn info(
    data: web::Data<AppState>,
    req: web::Json<AssetInfoInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let result = api
        .storage()
        .asset()
        .assets(&req.class_id.into(), &req.asset_id.into(), None)
        .await;
    let info = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetInfoOutput {
        info: match info {
            Some(info) => Some(AssetInfo {
                class_id: req.class_id.clone(),
                asset_id: req.asset_id.clone(),
                metadata: serde_json::from_slice(info.metadata.0.as_slice()).unwrap_or_default(),
            }),
            None => None,
        },
    }))
}

/// Update asset class metadata
#[cfg(not(feature = "keycloak"))]
pub async fn update_metadata(
    data: web::Data<AppState>,
    req: web::Json<UpdateMetadataInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    match update_asset_call(data, req, user_seed).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to update asset metadata"),
            description: format!("Error in asset::updateMetadata"),
        })),
    }    
}

/// Update asset class metadata
#[cfg(feature = "keycloak")]
pub async fn update_metadata(
    data: web::Data<AppState>,
    req: web::Json<UpdateMetadataInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                match update_asset_call(data, req, user_seed).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to update asset metadata"),
                        description: format!("Error in asset::updateMetadata"),
                    })),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::update_metadata"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::update_metadata"),
        })),
    }        
}

async fn update_asset_call(
    data: web::Data<AppState>,
    req: web::Json<UpdateMetadataInput>,
    seed: Seed,
) -> error::Result<web::Json<UpdateMetadataOutput>, HttpResponse> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .update_asset_metadata(req.class_id.into(), req.asset_id.into(), metadata)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::AssetMetadataUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(UpdateMetadataOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
            metadata: serde_json::from_slice(event.metadata.as_slice()).unwrap_or_default(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
            description: format!(""),
        })),
    }
}

/// Mint amount of asset to account
#[cfg(not(feature = "keycloak"))]
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    match mint_asset_call(data, req, user_seed, to).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to mint asset"),
            description: format!("Error in asset::mint"),
        })),
    } 
}

/// Mint amount of asset to account
#[cfg(feature = "keycloak")]
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let to = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let to = sp_core::crypto::AccountId32::from(to);
                match mint_asset_call(data, req, user_seed, to).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to mint asset"),
                        description: format!("Error in asset::mint"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::mint"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::mint"),
        })),
    }        
}

async fn mint_asset_call(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
    seed: Seed,
    to: AccountId32,
) -> error::Result<web::Json<MintOutput>, HttpResponse> {    
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .mint(
            to,
            req.class_id.into(),
            req.asset_id.into(),
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
        .find_first::<sugarfunge::asset::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(MintOutput {
            to: event.to.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::AssetMint"),
            description: format!(""),
        })),
    }
}

/// Burn amount of asset from account
#[cfg(not(feature = "keycloak"))]
pub async fn burn(
    data: web::Data<AppState>,
    req: web::Json<BurnInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    let from = sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    match burn_asset_call(data, req, user_seed, from).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to mint asset"),
            description: format!("Error in asset::mint"),
        })),
    } 
}

/// Burn amount of asset from account
#[cfg(feature = "keycloak")]
pub async fn burn(
    data: web::Data<AppState>,
    req: web::Json<BurnInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let from = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let from = sp_core::crypto::AccountId32::from(from);
                match burn_asset_call(data, req, user_seed, from).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to burn asset"),
                        description: format!("Error in asset::burn"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::burn"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::burn"),
        })),
    }        
}

async fn burn_asset_call(
    data: web::Data<AppState>,
    req: web::Json<BurnInput>,
    seed: Seed,
    from: AccountId32,
) -> error::Result<web::Json<BurnOutput>, HttpResponse> {    
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .burn(
            from,
            req.class_id.into(),
            req.asset_id.into(),
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
        .find_first::<sugarfunge::asset::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(BurnOutput {
            from: event.from.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::Burn"),
            description: format!(""),
        })),
    }
}

/// Get balance for given asset
#[cfg(not(feature = "keycloak"))]
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
) -> error::Result<HttpResponse> {
    
    let account = sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    match balance_asset_call(data, req, account).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to get asset balance"),
            description: format!("Error in asset::balance"),
        })),
    } 
}

/// Get balance for given asset
#[cfg(feature = "keycloak")]
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                match balance_asset_call(data, req, account).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to get asset balance"),
                        description: format!("Error in asset::balance"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::balance"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::balance"),
        })),
    }    
}

async fn balance_asset_call(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
    account: AccountId32,
) -> error::Result<web::Json<AssetBalanceOutput>> {
    let api = &data.api;
    let result = api
        .storage()
        .asset()
        .balances(&account, &req.class_id.into(), &req.asset_id.into(), None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(web::Json(AssetBalanceOutput {
        amount: amount.into(),
    }))
}

/// Transfer asset from to accounts
#[cfg(not(feature = "keycloak"))]
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    let from = sp_core::sr25519::Public::from_str(&req.from.as_str()).map_err(map_account_err)?;
    let from = sp_core::crypto::AccountId32::from(from);
    match transfer_asset_call(data, req, user_seed, from).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to transfer asset"),
            description: format!("Error in asset::transferFrom"),
        })),
    }    
}

/// Transfer asset from to accounts
#[cfg(feature = "keycloak")]
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let from = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let from = sp_core::crypto::AccountId32::from(from);
                match transfer_asset_call(data, req, user_seed, from).await {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to transfer asset"),
                        description: format!("Error in asset::transferFrom"),
                    })),
                }  
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in asset::transfer_from"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Not found user Attributes"),
                    description: format!("Error in asset::transfer_from"),
        })),
    }      
}

async fn transfer_asset_call(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
    seed: Seed,
    account_from: AccountId32,
) -> error::Result<web::Json<TransferFromOutput>, HttpResponse> {
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let result = api
        .tx()
        .asset()
        .transfer_from(
            account_from,
            account_to,
            req.class_id.into(),
            req.asset_id.into(),
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
        .find_first::<sugarfunge::asset::events::Transferred>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(web::Json(TransferFromOutput {
            from: event.from.into(),
            to: event.to.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Transferred"),
            description: format!(""),
        })),
    }
}