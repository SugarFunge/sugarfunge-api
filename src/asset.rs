use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::asset::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

/// Create an asset class for an account
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.owner).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let metadata = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .create_class(to, req.class_id.into(), metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::asset::events::ClassCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateClassOutput {
            class_id: event.class_id.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
        })),
    }
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
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .create_asset(req.class_id.into(), req.asset_id.into(), metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::asset::events::AssetCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateOutput {
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
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
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .mint(
            to,
            req.class_id.into(),
            req.asset_id.into(),
            req.amount.into(),
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::asset::events::Mint>()
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
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .burn(
            from,
            req.class_id.into(),
            req.asset_id.into(),
            req.amount.into(),
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::asset::events::Burn>()
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
        })),
    }
}

/// Get balance for given asset
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::sr25519::Public::from_str(&req.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .asset()
        .balances(account, req.class_id.into(), req.asset_id.into(), None)
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
    let api = data.api.lock().unwrap();
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
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::asset::events::Transferred>()
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
        })),
    }
}
