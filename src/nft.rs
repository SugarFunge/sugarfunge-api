use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct CreateNftInput {
    input: CreateNftArg,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNftArg {
    seed: String,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNftOutput {
    collection_id: u64,
    account: String,
}

/// Create a collection for an account
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateNftInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);

    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap_or_default();

    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .nft()
        .create_collection(metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;

    let result = result
        .find_event::<sugarfunge::nft::events::CollectionCreated>()
        .map_err(map_scale_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateNftOutput {
            collection_id: event.0,
            account: event.1.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::nft::events::CollectionCreated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct MintNftInput {
    input: MintNftArg,
}

#[derive(Serialize, Deserialize)]
pub struct MintNftArg {
    seed: String,
    collection_id: u64,
    metadata: serde_json::Value,
    quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MintNftOutput {
    collection_id: u64,
    token_ids: Vec<u64>,
    account: String,
}

/// Create a collection for an account
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintNftInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);

    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap_or_default();

    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .nft()
        .mint(req.input.collection_id, metadata, req.input.quantity)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;

    let result = result
        .find_event::<sugarfunge::nft::events::TokenMint>()
        .map_err(map_scale_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintNftOutput {
            collection_id: event.0,
            token_ids: event.1,
            account: event.2.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::nft::events::TokenMint"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct NftCollectionsInput {
    input: NftCollectionsArg,
}

#[derive(Serialize, Deserialize)]
pub struct NftCollectionsArg {
    collection_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NftCollectionsOutput {
    owner: String,
    total_supply: u128,
    deposit: u128,
    metadata: serde_json::Value,
}

/// Get collection info for collection id
pub async fn collections(
    data: web::Data<AppState>,
    req: web::Json<NftCollectionsInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .nft()
        .collections(req.input.collection_id, None)
        .await;
    let collection_info = result.map_err(map_subxt_err)?;
    if let Some(collection_info) = collection_info {
        let metadata = serde_json::from_slice(&collection_info.properties).unwrap_or_default();
        Ok(HttpResponse::Ok().json(NftCollectionsOutput {
            owner: collection_info.owner.to_string(),
            total_supply: collection_info.total_supply,
            deposit: collection_info.deposit,
            metadata,
        }))
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Invalid collection"),
        }))
    }
}

#[derive(Serialize, Deserialize)]
pub struct NftTokensInput {
    input: NftTokensArg,
}

#[derive(Serialize, Deserialize)]
pub struct NftTokensArg {
    collection_id: u64,
    token_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NftTokensOutput {
    owner: String,
    metadata: serde_json::Value,
}

/// Get token info for collection and token id
pub async fn tokens(
    data: web::Data<AppState>,
    req: web::Json<NftTokensInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .nft()
        .tokens(req.input.collection_id, req.input.token_id, None)
        .await;
    let token_info = result.map_err(map_subxt_err)?;
    if let Some(token_info) = token_info {
        let metadata = serde_json::from_slice(&token_info.metadata).unwrap_or_default();
        Ok(HttpResponse::Ok().json(NftTokensOutput {
            owner: token_info.owner.to_string(),
            metadata,
        }))
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Invalid collection"),
        }))
    }
}

#[derive(Serialize, Deserialize)]
pub struct NftOwnerInput {
    input: NftOwnerArg,
}

#[derive(Serialize, Deserialize)]
pub struct NftOwnerArg {
    account: String,
}

#[derive(Serialize, Deserialize)]
pub struct NftOwnerToken {
    collection_id: u64,
    token_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NftOwnerOutput {
    owner: String,
    tokens: Vec<NftOwnerToken>,
}

/// Get token info for collection and token id
pub async fn owner(
    data: web::Data<AppState>,
    req: web::Json<NftOwnerInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::sr25519::Public::from_str(&req.input.account);
    if let Ok(account) = account {
        let account = sp_core::crypto::AccountId32::from(account);

        let api = data.api.lock().unwrap();
        let result = api.storage().nft().tokens_by_owner_iter(None).await;
        let mut iter = result.map_err(map_subxt_err)?;

        // TODO: Filter by owner
        let mut tokens: Vec<NftOwnerToken> = vec![];
        while let Some((_key, (collection_id, token_id))) =
            iter.next().await.map_err(map_subxt_err)?
        {
            tokens.push(NftOwnerToken {
                collection_id,
                token_id,
            })
        }

        Ok(HttpResponse::Ok().json(NftOwnerOutput {
            owner: account.to_string(),
            tokens,
        }))
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Invalid account"),
        }))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransferNftInput {
    input: TransferNftArg,
}

#[derive(Serialize, Deserialize)]
pub struct TransferNftArg {
    seed: String,
    to: String,
    collection_id: u64,
    token_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TransferNftOutput {
    collection_id: u64,
    token_id: u64,
    from: String,
    to: String,
}

/// Transfer token to account
pub async fn transfer(
    data: web::Data<AppState>,
    req: web::Json<TransferNftInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);

    let account = sp_core::sr25519::Public::from_str(&req.input.to);
    if let Ok(account) = account {
        let account = sp_core::crypto::AccountId32::from(account);
        let api = data.api.lock().unwrap();
        let result = api
            .tx()
            .nft()
            .transfer(account, req.input.collection_id, req.input.token_id)
            .sign_and_submit_then_watch(&signer)
            .await
            .map_err(map_subxt_err)?;

        let result = result
            .find_event::<sugarfunge::nft::events::TokenTransferred>()
            .map_err(map_scale_err)?;

        match result {
            Some(event) => Ok(HttpResponse::Ok().json(TransferNftOutput {
                collection_id: event.0,
                token_id: event.1,
                from: event.2.to_string(),
                to: event.3.to_string(),
            })),
            None => Ok(HttpResponse::BadRequest().json(RequestError {
                message: json!("Failed to find sugarfunge::nft::events::TokenTransferred"),
            })),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Invalid account"),
        }))
    }
}
