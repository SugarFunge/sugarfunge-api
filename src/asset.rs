use crate::state::*;
use crate::sugarfunge;
use crate::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    seed: String,
    class_id: u64,
    metadata: serde_json::Value,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassOutput {
    class_id: u64,
    who: String,
}

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
        .create_class(to, req.class_id, metadata)
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
            class_id: event.class_id,
            who: event.who.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateInput {
    seed: String,
    class_id: u64,
    asset_id: u64,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOutput {
    class_id: u64,
    asset_id: u64,
    who: String,
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
        .create_asset(req.class_id, req.asset_id, metadata)
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
            class_id: event.class_id,
            asset_id: event.asset_id,
            who: event.who.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct MintInput {
    seed: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintOutput {
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}

/// Mint amount of asset to account
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .mint(to, req.class_id, req.asset_id, req.amount)
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
            to: event.to.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
            amount: event.amount,
            who: event.who.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::AssetMint"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct BurnInput {
    seed: String,
    from: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnOutput {
    from: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}

/// Burn amount of asset from account
pub async fn burn(
    data: web::Data<AppState>,
    req: web::Json<BurnInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let from = sp_core::sr25519::Public::from_str(&req.from).map_err(map_account_err)?;
    let from = sp_core::crypto::AccountId32::from(from);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .burn(from, req.class_id, req.asset_id, req.amount)
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
            from: event.from.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
            amount: event.amount,
            who: event.who.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::Burn"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceInput {
    account: String,
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    amount: u128,
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
        .balances(account, req.class_id, req.asset_id, None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetBalanceOutput { amount }))
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesInput {
    account: String,
    class_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalancesOutput {
    balances: Vec<AssetBalanceItemOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceItemOutput {
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

/// Get balances for given account
// pub async fn balances(
//     data: web::Data<AppState>,
//     req: web::Json<AssetBalancesInput>,
// ) -> error::Result<HttpResponse> {
//     let account =
//         sp_core::sr25519::Public::from_str(&req.account).map_err(map_account_err)?;
//     let account = sp_core::crypto::AccountId32::from(account);
//     let api = data.api.lock().unwrap();
//     // let result = api
//     //     .storage()
//     //     .asset()
//     //     .balances(account, req.class_id, req.asset_id, None)
//     //     .await;
//     let result = api.storage().asset().balances_iter(None).await.map_err(map_subxt_err)?;
//     result.next()
//     let amount = result.map_err(map_subxt_err)?;
//     Ok(HttpResponse::Ok().json(AssetBalancesOutput { amount }))
// }

#[derive(Serialize, Deserialize)]
pub struct TransferFromInput {
    seed: String,
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromOutput {
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
    who: String,
}

/// Transfer asset from to accounts
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from = sp_core::sr25519::Public::from_str(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let account_from = sp_core::crypto::AccountId32::from(account_from);
    let account_to = sp_core::crypto::AccountId32::from(account_to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .transfer_from(
            account_from,
            account_to,
            req.class_id,
            req.asset_id,
            req.amount,
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
            from: event.from.to_string(),
            to: event.to.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
            amount: event.amount,
            who: event.who.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Transferred"),
        })),
    }
}
