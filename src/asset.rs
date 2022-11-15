use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use std::str::FromStr;
use subxt::ext::sp_core;
use subxt::tx::PairSigner;
use sugarfunge_api_types::asset::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use codec::Decode;
use subxt::storage::address::{StorageHasher, StorageMapKey};
use sugarfunge_api_types::sugarfunge::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;

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
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
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
pub async fn update_metadata(
    data: web::Data<AppState>,
    req: web::Json<UpdateMetadataInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
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
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
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
            message: json!("Failed to find sugarfunge::balances::events::balance"),
            description: format!("Error in asset::balance"),
        })),
    }
}

/// Get balances for owner and maybe class
pub async fn balances(
    data: web::Data<AppState>,
    req: web::Json<AssetBalancesInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.account.as_str()).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = &data.api;

    let mut result_array = Vec::new();
    let mut query_key = sugarfunge::storage().asset().balances_root().to_bytes();
    // println!("query_key balances_root len: {}", query_key.len());
    StorageMapKey::new(&account, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    // println!("query_key account len: {}", query_key.len());
    if let Some(class_id) = req.class_id {
        let class_id: u64 = class_id.into();
        StorageMapKey::new(&class_id, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
        // println!("query_key class_id len: {}", query_key.len());
    }
    // if let Some(asset_id) = req.asset_id {
    //     let asset_id: u64 = asset_id.into();
    //     StorageMapKey::new(&asset_id, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    //     println!("query_key asset_id len: {}", query_key.len());
    // }

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        // let account_idx = 48;
        // let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        // let account_id = AccountId32::decode(&mut &account_key[..]);
        // let account_id = Account::from(account_id.unwrap());
        // let account_id = String::from(&account_id);
        // println!("account_id: {}", account_id);

        let class_idx = 96;
        let class_key = key.0.as_slice()[class_idx..(class_idx + 8)].to_vec();
        let class_id = u64::decode(&mut &class_key[..]);
        // println!("class_id: {:?}", class_id);

        let asset_idx = 120;
        let asset_key = key.0.as_slice()[asset_idx..(asset_idx + 8)].to_vec();
        let asset_id = u64::decode(&mut &asset_key[..]);
        // println!("asset_id: {:?}", asset_id);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = u128::decode(&mut &storage_data[..]);
            // println!(
            //     "Class_Id: {:?} AssetId: {:?}  Value: {:?}",
            //     class_id, asset_id, value
            // );
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
