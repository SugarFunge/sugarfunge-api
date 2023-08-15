use crate::state::*;
use crate::util::*;
use actix_web::Error;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use codec::Encode;
use hex::ToHex;
use serde_json::json;
use std::str::FromStr;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sugarfunge_api_types::bundle::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use sugarfunge_api_types::sugarfunge::runtime_types::sugarfunge_bundle::Bundle as BundleRuntime;

fn hash(s: &[u8]) -> sp_core::H256 {
    sp_io::hashing::blake2_256(s).into()
}

pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
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
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = &data.api;

    let call = sugarfunge::tx().bundle().register_bundle(
        req.class_id.into(),
        req.asset_id.into(),
        bundle_id,
        schema,
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
        .find_first::<sugarfunge::bundle::events::Register>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RegisterBundleOutput {
            who: event.who.into(),
            bundle_id: event.bundle_id.encode_hex(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
            description: String::new(),
        })),
    }
}

pub async fn mint_bundle(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from = subxt::utils::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = subxt::utils::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let bundle_id = sp_core::H256::from_str(req.bundle_id.as_str()).unwrap_or_default();
    let api = &data.api;

    let call = sugarfunge::tx().bundle().mint_bundle(
        account_from,
        account_to,
        bundle_id,
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
        .find_first::<sugarfunge::bundle::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintBundleOutput {
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.encode_hex(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Mint"),
            description: String::new(),
        })),
    }
}

pub async fn burn_bundle(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from = subxt::utils::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = subxt::utils::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let bundle_id = sp_core::H256::from_str(req.bundle_id.as_str()).unwrap_or_default();
    let api = &data.api;

    let call = sugarfunge::tx().bundle().burn_bundle(
        account_from,
        account_to,
        bundle_id,
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
        .find_first::<sugarfunge::bundle::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BurnBundleOutput {
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.encode_hex(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Burn"),
            description: String::new(),
        })),
    }
}

pub async fn get_bundles_id(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;

    let mut result_array = Vec::new();
    let query_key = sugarfunge::storage()
        .bundle()
        .asset_bundles_root()
        .to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let class_idx = 48;
        let class_key = key.0.as_slice()[class_idx..(class_idx + 8)].to_vec();
        let class_id = u64::decode(&mut &class_key[..]).unwrap();
        // println!("class_id: {}", class_id);

        let asset_idx = 72;
        let asset_key = key.0.as_slice()[asset_idx..(asset_idx + 8)].to_vec();
        let asset_id = u64::decode(&mut &asset_key[..]).unwrap();
        // println!("asset_id: {}", asset_id);

        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = sp_core::H256::decode(&mut &storage_data[..]).unwrap();
            let bundle_id = value.encode_hex();

            let item = BundleItem {
                class_id: class_id.into(),
                asset_id: asset_id.into(),
                bundle_id,
            };
            result_array.push(item);
        }
    }

    Ok(HttpResponse::Ok().json(GetBundles {
        bundles: result_array,
    }))
}

pub async fn verify_bundle_exist(
    data: &web::Data<AppState>,
    bundle_id_value: BundleId,
) -> Result<bool, Error> {
    let api = &data.api;

    let query_key = sugarfunge::storage()
        .bundle()
        .asset_bundles_root()
        .to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    for key in keys.iter() {
        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = sp_core::H256::decode(&mut &storage_data[..]).unwrap();
            let bundle_id: BundleId = value.encode_hex();

            if bundle_id.as_str() == bundle_id_value.as_str() {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

pub async fn get_bundles_data(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;

    let mut result_array = Vec::new();
    let query_key = sugarfunge::storage()
        .bundle()
        .bundles_root()
        .to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let bundle_idx = 48;
        let bundle_key = key.0.as_slice()[bundle_idx..].to_vec();
        let bundle_id = sp_core::H256::decode(&mut &bundle_key[..]).unwrap();
        let bundle_id_value: BundleId = bundle_id.encode_hex();

        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = BundleRuntime::<
                u64,
                u64,
                (Vec<u64>, Vec<Vec<u64>>, Vec<Vec<u128>>),
                AccountId32,
                Vec<u8>,
            >::decode(&mut &storage_data[..])
            .unwrap();

            let item = BundleDataItem {
                bundle_id: bundle_id_value,
                creator: value.creator.into(),
                class_id: value.class_id.into(),
                asset_id: value.asset_id.into(),
                metadata: serde_json::from_slice(value.metadata.as_slice()).unwrap_or_default(),
                schema: BundleSchema {
                    class_ids: get_schema_class_ids(value.schema.0),
                    asset_ids: get_schema_vec_asset_ids(value.schema.1),
                    amounts: get_schema_vec_amounts(value.schema.2),
                },
            };
            result_array.push(item);
        }
    }
    Ok(HttpResponse::Ok().json(GetBundlesData {
        bundles: result_array,
    }))
}

pub fn get_schema_class_ids(class_ids: Vec<u64>) -> Vec<ClassId> {
    return class_ids.iter().map(|value| (*value).into()).collect();
}

pub fn get_schema_asset_ids(asset_ids: Vec<u64>) -> Vec<AssetId> {
    return asset_ids.iter().map(|value| (*value).into()).collect();
}

pub fn get_schema_amounts(amounts: Vec<u128>) -> Vec<Balance> {
    return amounts.iter().map(|value| (*value).into()).collect();
}

pub fn get_schema_vec_asset_ids(asset_ids: Vec<Vec<u64>>) -> Vec<Vec<AssetId>> {
    return asset_ids
        .iter()
        .map(|value| get_schema_asset_ids(value.to_vec()))
        .collect();
}

pub fn get_schema_vec_amounts(amounts: Vec<Vec<u128>>) -> Vec<Vec<Balance>> {
    return amounts
        .iter()
        .map(|value| get_schema_amounts(value.to_vec()))
        .collect();
}
