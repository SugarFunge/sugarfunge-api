use crate::bundle::*;
use crate::state::AppState;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Encode;
use contract_integration::admin_calls::*;
use hex::ToHex;
use serde_json::json;
use sp_core::U256;
use subxt::ext::sp_core::sr25519::Public;
use subxt::ext::sp_core::Pair;
use subxt::ext::sp_runtime::traits::IdentifyAccount;
use subxt::ext::sp_runtime::AccountId32;
use subxt::tx::PairSigner;
use sugarfunge_api_types::contract::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;

const LABOR_TOKEN_CLASS_ID: u64 = 100;
const LABOR_TOKEN_ASSET_ID: u64 = 100;
const LABOR_TOKEN_VALUE: u128 = 1;

const CHALLENGE_TOKEN_CLASS_ID: u64 = 110;
const CHALLENGE_TOKEN_ASSET_ID: u64 = 100;
const CHALLENGE_TOKEN_VALUE: u128 = 1;

const CLAIMED_TOKEN_CLASS_ID: u64 = 120;
const CLAIMED_TOKEN_ASSET_ID: u64 = 100;

pub async fn contract_mint_to(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = mint_to(req.account_address.as_str(), U256::from(req.amount)).await;
    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::MintTo"),
            description: format!(""),
        })),
    }
}

pub async fn contract_burn_from(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = burn_from(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::BurnFrom"),
            description: format!(""),
        })),
    }
}

pub async fn contract_transfer(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = transfer(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Transfer"),
            description: format!(""),
        })),
    }
}

pub async fn contract_total_supply() -> error::Result<HttpResponse> {
    let result = total_supply().await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractTotalSupplyOutput {
            total_supply: remove_decimals_from_u256(event.total_supply, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::TotalSupply"),
            description: format!(""),
        })),
    }
}

pub async fn contract_allowance(
    req: web::Json<ContractAllowanceInput>,
) -> error::Result<HttpResponse> {
    let result = allowance(req.owner_address.as_str(), req.spender_address.as_str()).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractAllowanceOutput {
            allowance: remove_decimals_from_u256(event.allowance, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Allowance"),
            description: format!(""),
        })),
    }
}

pub async fn contract_increase_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = increase_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::IncreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn contract_decrease_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = decrease_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::DecreaseAllowance"),
            description: format!(""),
        })),
    }
}

fn hash(s: &[u8]) -> sp_core::H256 {
    sp_io::hashing::blake2_256(s).into()
}

pub async fn convert_to_fula(
    data: web::Data<AppState>,
    req: web::Json<ConvertFulaInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let paired = get_pair_from_seed(&req.seed)?;
    let account: Public = paired.public().into();
    let account = account.into_account();
    let account = Account::from(format!("{}", account));
    let account_from = AccountId32::try_from(&account).map_err(map_account_err)?;
    let account_to = AccountId32::try_from(&account).map_err(map_account_err)?;

    // Create the bundle schema

    // println!("1. CREATING SCHEMA");

    let schema = (
        BoundedVec(vec![LABOR_TOKEN_CLASS_ID, CHALLENGE_TOKEN_CLASS_ID]),
        BoundedVec(vec![
            BoundedVec(vec![LABOR_TOKEN_ASSET_ID]),
            BoundedVec(vec![CHALLENGE_TOKEN_ASSET_ID]),
        ]),
        BoundedVec(vec![
            BoundedVec(vec![LABOR_TOKEN_VALUE]),
            BoundedVec(vec![CHALLENGE_TOKEN_VALUE]),
        ]),
    );
    let bundle_id = hash(&schema.encode());
    let api = &data.api;

    // Verify if the Bundle_id exist
    // println!("2. VERIFYING IF THE BUNDLE ID EXIST");

    if let Ok(verification) = verify_bundle_exist(&data, bundle_id.encode_hex()).await {
        // If it doesn't exist, register the bundle
        if !verification {
            // println!("3. THE BUNDLE ID DOESN'T EXISTS");
            let call = sugarfunge::tx().bundle().register_bundle(
                CLAIMED_TOKEN_CLASS_ID.into(),
                CLAIMED_TOKEN_ASSET_ID.into(),
                bundle_id,
                schema,
                BoundedVec(vec![]),
            );

            let _result = api
                .tx()
                .sign_and_submit_then_watch(&call, &signer, Default::default())
                .await
                .map_err(map_subxt_err)?
                .wait_for_finalized_success()
                .await
                .map_err(map_sf_err)?;
            // println!("4. BUNDLE CREATED");
        };

        // If exist, continue
        // println!("5. THE BUNDLE ID EXISTS");
        // Mint the Bundle with the bundle_id

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
            Some(_) => {
                // If the bundle mint is successful, execute the contract mint
                // println!("6. BUNDLE MINTED SUCCESSFULLY");
                let result = mint_to(
                    req.wallet_account.as_str(),
                    U256::from(u128::from(req.amount)),
                )
                .await;
                match result {
                    Ok(event) => Ok(HttpResponse::Ok().json(event)),
                    Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to execute the contract_event::MintTo"),
                        description: format!(""),
                    })),
                }
            }
            // If the bundle mint failed, show an error to try again
            None => Ok(HttpResponse::BadRequest().json(RequestError {
                message: json!("Failed to execute the Bundle Mint"),
                description: format!(""),
            })),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to verify if the Bundle ID exist"),
            description: format!(""),
        }))
    }
}
