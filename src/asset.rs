use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use sp_core::crypto::AccountId32;
use sugarfunge_api_types::primitives::*;
use std::str::FromStr;
use subxt::tx::PairSigner;
use sugarfunge_api_types::asset::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;
use codec::Decode;
use subxt::storage::address::{StorageHasher, StorageMapKey};

#[cfg(feature = "keycloak")]
use crate::config::Config;
#[cfg(feature = "keycloak")]
use crate::user;
#[cfg(feature = "keycloak")]
use actix_web_middleware_keycloak_auth::KeycloakClaims;
#[cfg(feature = "keycloak")]
use sp_core::Pair;
#[cfg(feature = "keycloak")]
use subxt::ext::sp_runtime::traits::IdentifyAccount;

#[cfg(not(feature = "keycloak"))]
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let user_seed = Seed::from(req.seed.clone());
    let to = sp_core::sr25519::Public::from_str(&req.owner.as_str()).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    match create_class_call(data, req, user_seed, to).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;

    let call = sugarfunge::tx()
        .asset()
        .create_class(to, req.class_id.into(), metadata);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::ClassCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateClassOutput {
            class_id: event.class_id.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
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

    let call = sugarfunge::storage().asset().classes(&req.class_id.into());

    let result = api.storage().fetch(&call, None).await;
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
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let call =
        sugarfunge::tx()
            .asset()
            .create_asset(req.class_id.into(), req.asset_id.into(), metadata);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::AssetCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::AssetCreated"),
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

    let call = sugarfunge::storage()
        .asset()
        .assets(&req.class_id.into(), &req.asset_id.into());

    let result = api.storage().fetch(&call, None).await;
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
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;
    let call = sugarfunge::tx().asset().update_asset_metadata(
        req.class_id.into(),
        req.asset_id.into(),
        metadata,
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::AssetMetadataUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(UpdateMetadataOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
            metadata: serde_json::from_slice(event.metadata.as_slice()).unwrap_or_default(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
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
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                let to =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let to = sp_core::crypto::AccountId32::from(to);
                match mint_asset_call(data, req, user_seed, to).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let call = sugarfunge::tx().asset().mint(
        to,
        req.class_id.into(),
        req.asset_id.into(),
        req.amount.into(),
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintOutput {
            to: event.to.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Mint"),
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
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                let from =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let from = sp_core::crypto::AccountId32::from(from);
                match burn_asset_call(data, req, user_seed, from).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let call = sugarfunge::tx().asset().burn(
        from,
        req.class_id.into(),
        req.asset_id.into(),
        req.amount.into(),
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BurnOutput {
            from: event.from.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Burn"),
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
    let account =
        sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    match balance_asset_call(data, req, account).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                let account =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                match balance_asset_call(data, req, account).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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
) -> error::Result<HttpResponse, actix_web::Error> {
    let api = &data.api;
    let call = sugarfunge::storage().asset().balances(
        &account,
        &req.class_id.into(),
        &req.asset_id.into(),
    );

    let result = api.storage().fetch(&call, None).await;
    
    let amount = result.map_err(map_subxt_err)?;
    match amount {
        Some(amount) => Ok(HttpResponse::Ok().json(AssetBalanceOutput {
            amount: amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find asset::getBalance"),
            description: format!("Error in asset::balance"),
        })),
    } 
}

/// Get balances for owner and maybe class
#[cfg(not(feature = "keycloak"))]
pub async fn balances(
    data: web::Data<AppState>,
    req: web::Json<AssetBalancesInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    match balances_asset_call(data, req, account).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
    }
}

/// Get balances for owner and maybe class
#[cfg(feature = "keycloak")]
pub async fn balances(
    data: web::Data<AppState>,
    req: web::Json<AssetBalancesInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                //let user_seed = Seed::from(response.seed.clone().unwrap());              
                match balances_asset_call(data, req, account).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
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

async fn balances_asset_call(
    data: web::Data<AppState>,
    req: web::Json<AssetBalancesInput>,
    account: AccountId32,
) -> error::Result<HttpResponse, actix_web::Error> {
    let api = &data.api;

    let mut result_array = Vec::new();
    let mut query_key = sugarfunge::storage().asset().balances_root().to_bytes();

    StorageMapKey::new(&account, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    
    if let Some(class_id) = req.class_id {
        let class_id: u64 = class_id.into();
        StorageMapKey::new(&class_id, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);        
    }
    
    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    for key in keys.iter() {

        let class_idx = 96;
        let class_key = key.0.as_slice()[class_idx..(class_idx + 8)].to_vec();
        let class_id = u64::decode(&mut &class_key[..]);

        let asset_idx = 120;
        let asset_key = key.0.as_slice()[asset_idx..(asset_idx + 8)].to_vec();
        let asset_id = u64::decode(&mut &asset_key[..]);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = u128::decode(&mut &storage_data[..]);
            let item = AssetBalanceItemOutput{
                class_id: ClassId::from(class_id.unwrap()),
                asset_id: AssetId::from(asset_id.unwrap()),
                amount: Balance::from(value.unwrap()),
            };
            result_array.push(item);
        }
    }

    Ok(HttpResponse::Ok().json(AssetBalancesOutput {
        balances: result_array,
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
        Ok(response) => Ok(response),
        Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
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
                let from =
                    sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let from = sp_core::crypto::AccountId32::from(from);
                match transfer_asset_call(data, req, user_seed, from).await {
                    Ok(response) => Ok(response),
                    Err(e) => Ok(HttpResponse::BadRequest().json(actixweb_err_to_json(e))),
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found seed in user Attributes"),
                    description: format!("Error in asset::transfer_from"),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in asset::transfer_from"),
        })),
    }
}

async fn transfer_asset_call(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
    seed: Seed,
    account_from: AccountId32,
) -> error::Result<HttpResponse, actix_web::Error> {
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let pair = get_pair_from_seed(&seed)?;
    let signer = PairSigner::new(pair);
    let api = &data.api;
    let call = sugarfunge::tx().asset().transfer_from(
        account_from,
        account_to,
        req.class_id.into(),
        req.asset_id.into(),
        req.amount.into(),
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::asset::events::Transferred>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(TransferFromOutput {
            from: event.from.into(),
            to: event.to.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Transferred"),
            description: format!(""),
        })),
    }
}
