use crate::bundle::*;
use crate::config;
use crate::state::AppState;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Encode;
use contract_integration::types::ReceiptOutput;
use dotenv::dotenv;
use hex::ToHex;
use serde_json::json;
use subxt::ext::sp_core::sr25519::Public;
use subxt::ext::sp_core::Pair;
use subxt::ext::sp_runtime::traits::IdentifyAccount;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sugarfunge_api_types::contract::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::bounded_collections::bounded_vec::BoundedVec;

// Functions to call the {goerli/convert} endpoint of the fula-contract-api
pub async fn goerli_convert_to_fula_endpoint(
    data: web::Data<AppState>,
    req: web::Json<ConvertFulaInput>,
) -> error::Result<HttpResponse> {
    convert_to_fula_call(data, req, "goerli/mint").await
}

// Functions to call the {mumbai/convert} endpoint of the fula-contract-api
pub async fn mumbai_convert_to_fula_endpoint(
    data: web::Data<AppState>,
    req: web::Json<ConvertFulaInput>,
) -> error::Result<HttpResponse> {
    convert_to_fula_call(data, req, "mumbai/mint").await
}

pub async fn convert_to_fula_call(
    data: web::Data<AppState>,
    req: web::Json<ConvertFulaInput>,
    route: &'static str,
) -> error::Result<HttpResponse> {
    dotenv().ok();
    let env = config::init();

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
        BoundedVec(vec![env.labor_token_class_id, env.challenge_token_class_id]),
        BoundedVec(vec![
            BoundedVec(vec![env.labor_token_asset_id]),
            BoundedVec(vec![env.challenge_token_asset_id]),
        ]),
        BoundedVec(vec![
            BoundedVec(vec![env.labor_token_value]),
            BoundedVec(vec![env.challenge_token_value]),
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
                env.claimed_token_class_id.into(),
                env.claimed_token_asset_id.into(),
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

        // Health loop to ensure that the fula-contract-api is running

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
                let result: Result<ReceiptOutput, _> = request(
                    route,
                    ContractTransactionInput {
                        account_address: String::from(req.wallet_account.as_str()),
                        amount: u128::from(req.amount),
                    },
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

        // If exist, continue
        // println!("5. THE BUNDLE ID EXISTS");
        // Mint the Bundle with the bundle_id
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to verify if the Bundle ID exist"),
            description: format!(""),
        }))
    }
}
