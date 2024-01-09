use std::str::FromStr;

use crate::fula::get_vec_cids_from_input;
use crate::fula::get_vec_cids_from_node;
use crate::fula::transform_vec_uploader_data_runtime_to_vec_uploader_data;
use crate::fula::verify_contains_storer;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use serde_json::json;
use sp_core::sr25519::Public;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sugarfunge_api_types::challenge::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_fula::{
    Challenge as ChallengeRuntime, ClaimData as ClaimRuntime, Manifest as ManifestRuntime,
};
use futures::stream::StreamExt;

pub async fn generate_challenge(
    data: web::Data<AppState>,
    req: web::Json<GenerateChallengeInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);
    log::info!("sugarfunge-api generate_challenge: started");
    let api = &data.api;

    let call = sugarfunge::tx().fula().generate_challenge();
    log::info!("sugarfunge-api generate_challenge: call created");
    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    log::info!("sugarfunge-api generate_challenge: result1: {:#?}", result);
    let result = result
        .find_first::<sugarfunge::fula::events::Challenge>()
        .map_err(map_subxt_err)?;
    log::info!("sugarfunge-api generate_challenge: result2: {:#?}", result);
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(GenerateChallengeOutput {
            challenger: event.challenger.into(),
            challenged: event.challenged.into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            state: event.state.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::GenerateChallenge"),
            description: format!(""),
        })),
    }
}

pub async fn verify_challenge(
    data: web::Data<AppState>,
    req: web::Json<VerifyChallengeInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cids.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx().fula().verify_challenge(
        req.pool_id.into(),
        cids,
        req.class_id.into(),
        req.asset_id.into(),
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::VerifiedChallenges>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json({
            VerifyChallengeOutput {
                account: event.challenged.into(),
                successful_cids: get_vec_cids_from_node(event.successful),
                failed_cids: get_vec_cids_from_node(event.failed),
            }
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::VerifyChallenge"),
            description: format!(""),
        })),
    }
}

pub async fn mint_labor_tokens(
    data: web::Data<AppState>,
    req: web::Json<MintLaborTokensInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().fula().mint_labor_tokens(
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
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::MintedLaborTokens>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintLaborTokensOutput {
            account: event.account.into(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            amount: (event.amount as u128).into(),
            calculated_amount: (event.calculated_amount as u128).into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::MintedLaborTokens"),
            description: format!(""),
        })),
    }
}

pub async fn verify_pending_challenge(
    data: web::Data<AppState>,
    req: web::Json<VerifyPendingChallengeInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result = false;
    let requested_public = Public::from_str(&req.account).map_err(map_account_err)?;
    let requested_account_id = AccountId32::from(requested_public.0);

    let query_key = sugarfunge::storage()
        .fula()
        .challenge_requests_iter1(requested_account_id.clone())
        .to_root_bytes();

    // println!("query_key account_to len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>
    // println!("Obtained keys:");
    for key in keys.iter() {
        let account_idx = 48;
        let account_key_slice = &key.as_slice()[account_idx..(account_idx + 32)];
        let account_id = AccountId32::from(<[u8; 32]>::try_from(account_key_slice)
            .map_err(map_try_from_slice_err)?);

        if account_id == requested_account_id {
            result = true;
        }
    }
    Ok(HttpResponse::Ok().json(VerifyPendingChallengeOutput {
        account: req.account.clone(),
        pending: result,
    }))
}

pub async fn verify_file_size(
    data: web::Data<AppState>,
    req: web::Json<VerifyFileSizeInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();
    let requested_public = Public::from_str(&req.account).map_err(map_account_err)?;
    let _requested_account_id = AccountId32::from(requested_public.0);

    let query_key = sugarfunge::storage()
        .fula()
        .manifests_iter()
        .to_root_bytes();

    // println!("query_key account_to len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;
    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        let cid_idx = 68;
        let cid_key = key.as_slice()[cid_idx..].to_vec();
        let cid_id = String::decode(&mut &cid_key[..]);
        let cid_id = cid_id.unwrap();
        // println!("cid_id: {:?}", cid_id);

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ManifestRuntime::<AccountId32, Vec<u8>>::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            let uploaders_data =
                transform_vec_uploader_data_runtime_to_vec_uploader_data(value.users_data);

            if let Ok(contained_value) =
                verify_contains_storer(uploaders_data.to_owned(), req.account.clone())
            {
                if contained_value {
                    if let None = value.size {
                        result_array.push(Cid::from(cid_id))
                    }
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(VerifyFileSizeOutput {
        account: req.account.clone(),
        cids: result_array,
    }))
}

pub async fn provide_file_size(
    data: web::Data<AppState>,
    req: web::Json<ProvideFileSizeInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cids.to_vec());

    let api = &data.api;

    let call =
        sugarfunge::tx()
            .fula()
            .update_file_sizes(cids, req.pool_id.into(), req.sizes.to_vec());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::UpdateFileSizesOutput>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ProvideFileSizeOutput {
            account: event.account.into(),
            pool_id: event.pool_id.into(),
            cids: get_vec_cids_from_node(event.cids),
            sizes: event.sizes.to_vec(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UpdateFileSizesOutput"),
            description: format!(""),
        })),
    }
}

pub async fn get_challenges(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage()
        .fula()
        .challenge_requests_iter()
        .to_root_bytes();

    // println!("query_key account_to len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        let account_idx = 48;
        let account_key = key.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ChallengeRuntime::<AccountId32>::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            result_array.push(ChallengeData {
                challenger: value.challenger.into(),
                challenged: account_id,
                state: value.challenge_state.into(),
            })
        }
    }
    Ok(HttpResponse::Ok().json(GetChallengesOutput {
        challenges: result_array,
    }))
}

pub async fn get_claims(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().fula().claims_iter().to_root_bytes();

    // println!("query_key account_to len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        let account_idx = 48;
        let account_key = key.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ClaimRuntime::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            result_array.push(ClaimData {
                account: account_id,
                minted_labor_tokens: value.minted_labor_tokens.into(),
                expected_labor_tokens: value.expected_labor_tokens.into(),
                minted_challenge_tokens: value.challenge_tokens.into(),
            })
        }
    }
    Ok(HttpResponse::Ok().json(GetClaimDataOutput {
        claims: result_array,
    }))
}
