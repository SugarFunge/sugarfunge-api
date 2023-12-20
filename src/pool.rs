use crate::account;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use codec::Encode;
use serde_json::json;
use std::str::FromStr;
use subxt::ext::sp_core::sr25519::Public;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sugarfunge_api_types::pool::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use sugarfunge_api_types::sugarfunge::runtime_types::fula_pool::Pool as PoolRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::fula_pool::PoolRequest as PoolRequestRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::fula_pool::User as UserRuntime;

pub async fn create_pool(
    data: web::Data<AppState>,
    req: web::Json<CreatePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let pool_name = String::from(&req.pool_name).into_bytes();

    let region = &req.region;
    let region: Region = region.into();
    let region: Vec<u8> = region.into();

    let peer_id = String::from(&req.peer_id).into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().create(pool_name, region, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::PoolCreated>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreatePoolOutput {
            owner: transform_option_account_value(event.owner).into(),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::PoolCreated"),
            description: format!(""),
        })),
    }
}

pub async fn leave_pool(
    data: web::Data<AppState>,
    req: web::Json<LeavePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().leave_pool(req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::ParticipantLeft>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(LeavePoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn join_pool(
    data: web::Data<AppState>,
    req: web::Json<JoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let peer_id = String::from(&req.peer_id).into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().join(req.pool_id.into(), peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::JoinRequested>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(JoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn cancel_join_pool(
    data: web::Data<AppState>,
    req: web::Json<CancelJoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().cancel_join(req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::RequestWithdrawn>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CancelJoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::RequestWithdrawn"),
            description: format!(""),
        })),
    }
}

pub async fn vote(
    data: web::Data<AppState>,
    req: web::Json<VoteInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let account = AccountId32::try_from(&req.account).map_err(map_account_err)?;

    let peer_id = String::from(&req.peer_id).into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx()
        .pool()
        .vote(req.pool_id.into(), account, req.vote_value, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::VotingResult>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(VoteOutput {
            account: event.account.into(),
            pool_id: event.pool_id.into(),
            result: String::from_utf8(event.result).unwrap_or_default().into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::Accepted"),
            description: format!(""),
        })),
    }
}

pub async fn get_all_pools(
    data: web::Data<AppState>,
    req: web::Json<GetAllPoolInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().pool().pools_root().to_root_bytes();
    // println!("query_key pool_root len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = PoolRuntime::decode(&mut &storage_data[..]);
            let pool_value = value.unwrap();

            let storage = pool_value.participants.0;

            let mut storage_vec: Vec<Account> = Vec::new();

            for storer in storage {
                let current_account = Account::try_from(storer).unwrap();
                storage_vec.push(current_account);
            }

            let pool_region = String::from_utf8(pool_value.region.0).unwrap_or_default();

            if let Some(region) = &req.region {
                if *region != pool_region {
                    meet_requirements = false;
                }
            }

            if meet_requirements {
                result_array.push(PoolData {
                    pool_id: pool_id.into(),
                    pool_name: String::from_utf8(pool_value.name.0)
                        .unwrap_or_default()
                        .into(),
                    region: pool_region,
                    creator: transform_option_account_value(pool_value.owner),
                    parent: transform_option_pool_value(pool_value.parent),
                    participants: storage_vec,
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolsOutput {
        pools: result_array,
    }))
}

pub async fn get_all_pool_requests(
    data: web::Data<AppState>,
    req: web::Json<GetAllPoolRequestInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage()
        .pool()
        .pool_requests_root()
        .to_root_bytes();
    // println!("query_key pool_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        query_key.extend(subxt::ext::sp_core::blake2_128(&key_value.encode()));
        // println!("query_key pool_id len: {}", query_key.len());
    }

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        let account_idx = 68;
        let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = PoolRequestRuntime::decode(&mut &storage_data[..]);
            let poolrequest_value = value.unwrap();

            let voters = poolrequest_value.voted.0;

            let mut voters_vec: Vec<Account> = Vec::new();

            for voter in voters {
                let current_account = Account::try_from(voter).unwrap();
                voters_vec.push(current_account);
            }

            if let Some(account_filter) = req.account.clone() {
                if AccountId32::from(
                    Public::from_str(&account_id.as_str()).map_err(map_account_err)?,
                ) != AccountId32::from(
                    Public::from_str(&account_filter.as_str()).map_err(map_account_err)?,
                ) {
                    meet_requirements = false;
                }
            }

            if meet_requirements {
                result_array.push(PoolRequestData {
                    pool_id: pool_id.into(),
                    account: account_id,
                    voted: voters_vec,
                    positive_votes: poolrequest_value.positive_votes,
                    peer_id: String::from_utf8(poolrequest_value.peer_id.0)
                        .unwrap_or_default()
                        .into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolRequestsOutput {
        poolrequests: result_array,
    }))
}

pub async fn get_all_pool_users(
    data: web::Data<AppState>,
    req: web::Json<GetAllPoolUsersInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().pool().users_root().to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys = storage
        .fetch_keys(&query_key, 1000, None)
        .await
        .map_err(map_subxt_err)?;

    for key in keys.iter() {
        let account_idx = 48;
        let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());

        if let Some(storage_data) = storage.fetch_raw(&key.0).await.map_err(map_subxt_err)? {
            let value = UserRuntime::<BoundedVec<u8>>::decode(&mut &storage_data[..]);
            let user_value = value.unwrap();
            let input_pool_id = req.pool_id.map(|id| id);
            let input_request_pool_id = req.request_pool_id.map(|id| id);

            let mut meet_requirements = true;

            // Apply the filtering logic based on pool_id and request_pool_id
            if input_pool_id.is_some() {
                let input_pool_id_u32: u32 = input_pool_id.unwrap().into();
                meet_requirements &= user_value.pool_id == Some(input_pool_id_u32);
            }
            if input_request_pool_id.is_some() {
                let input_request_pool_id_u32: u32 = input_request_pool_id.unwrap().into();
                meet_requirements &= user_value.request_pool_id == Some(input_request_pool_id_u32);
            }

            if input_request_pool_id.is_some() && input_pool_id.is_some() {
                let input_pool_id_u32: u32 = input_pool_id.unwrap().into();
                let input_request_pool_id_u32: u32 = input_request_pool_id.unwrap().into();
                meet_requirements = (user_value.request_pool_id == Some(input_request_pool_id_u32)) || (user_value.pool_id == Some(input_pool_id_u32));
            }

            // Additional check for account value
            if let Some(account_value) = req.account.clone() {
                meet_requirements &= AccountId32::from(
                    Public::from_str(&account_value.as_str()).map_err(map_account_err)?,
                ) == AccountId32::from(
                    Public::from_str(&account_id.as_str()).map_err(map_account_err)?,
                );
            }

            if meet_requirements {
                result_array.push(PoolUserData {
                    account: account_id,
                    pool_id: transform_option_pool_value(user_value.pool_id),
                    request_pool_id: transform_option_pool_value(user_value.request_pool_id),
                    peer_id: String::from_utf8(user_value.peer_id.0)
                        .unwrap_or_default()
                        .into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolUsersOutput {
        users: result_array,
    }))
}

