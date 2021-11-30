use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap();

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
    asset_ids: Vec<u64>,
    account: String,
}

/// Create a collection for an account
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintNftInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);

    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap();

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
            asset_ids: event.1,
            account: event.2.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::nft::events::TokenMint"),
        })),
    }
}
