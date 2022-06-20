use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::asset::*;
// use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

/// Create an asset class for an account
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.owner.as_str()).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
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
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
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
        Some(event) => Ok(HttpResponse::Ok().json(CreateOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
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
pub async fn update_metadata(
    data: web::Data<AppState>,
    req: web::Json<UpdateMetadataInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
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
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
        Some(event) => Ok(HttpResponse::Ok().json(MintOutput {
            to: event.to.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::AssetMint"),
            description: format!(""),
        })),
    }
}

/// Burn amount of asset from account
pub async fn burn(
    data: web::Data<AppState>,
    req: web::Json<BurnInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let from = sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
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
        Some(event) => Ok(HttpResponse::Ok().json(BurnOutput {
            from: event.from.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::Burn"),
            description: format!(""),
        })),
    }
}

/// Get balance for given asset
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = &data.api;
    let result = api
        .storage()
        .asset()
        .balances(&account, &req.class_id.into(), &req.asset_id.into(), None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetBalanceOutput {
        amount: amount.into(),
    }))
}

/// Transfer asset from to accounts
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from =
        sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
