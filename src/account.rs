use crate::state::*;
use crate::user;
use crate::util::*;
use actix_web::{error, web, HttpRequest, HttpResponse/*, http::header, http::StatusCode*/};
use actix_web_middleware_keycloak_auth::KeycloakClaims;
use rand::prelude::*;
use serde_json::json;
use sp_core::Pair;
use std::str::FromStr;
use subxt::{sp_runtime::traits::IdentifyAccount, PairSigner};
use sugarfunge_api_types::account::*;
use sugarfunge_api_types::config::Config;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;


/// Generate a unique seed and its associated account
pub async fn create(_req: HttpRequest) -> error::Result<HttpResponse> {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let seed = hex::encode(seed);
    let seed = format!("//{}", seed);
    let seed = Seed::from(seed);
    let pair = get_pair_from_seed(&seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(CreateAccountOutput {
        seed,
        account: Account::from(format!("{}", account)),
    }))
}

/// Compute account from seed
pub async fn seeded(
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let account = pair.public().into_account();
                Ok(HttpResponse::Ok().json(SeededAccountOutput {
                    seed: user_seed.clone(),
                    account: Account::from(format!("{}", account)),
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in account::fund"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::fund"),
        }))
    }
}

/// Fund a given account with amount
pub async fn fund(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let funder = Seed::from("//Alice".to_string());
                let funder_pair = get_pair_from_seed(&funder)?;
                let signer = PairSigner::new(funder_pair);
                let account= sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                let account = subxt::sp_runtime::MultiAddress::Id(account);
                let amount_input = req.amount;
                let api = &data.api;
                let result = api
                    .tx()
                    .balances()
                    .transfer(account, amount_input.into())
                    .map_err(map_subxt_err)?
                    .sign_and_submit_then_watch(&signer, Default::default())
                    .await
                    .map_err(map_subxt_err)?
                    .wait_for_finalized_success()
                    .await
                    .map_err(map_sf_err)?;
                let result = result
                    .find_first::<sugarfunge::balances::events::Transfer>()
                    .map_err(map_subxt_err)?;
                match result {
                    Some(event) => Ok(HttpResponse::Ok().json(FundAccountOutput {
                        from: event.from.into(),
                        to: event.to.into(),
                        amount: event.amount.into(),
                    })),
                    None => Ok(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to find sugarfunge::balances::events::Transfer"),
                        description: format!("Error in account::fund"),
                    })),
                } 
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in account::fund"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::fund"),
        }))
    }
}

/// Get balance for given account
pub async fn balance(
    data: web::Data<AppState>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                let api = &data.api;
                let result = api.storage().system().account(&account, None).await;
                let data = result.map_err(map_subxt_err)?;
                Ok(HttpResponse::Ok().json(AccountBalanceOutput {
                    balance: data.data.free.into(),
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in account::fund"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::fund"),
        }))
    }
}

/// Check if account exists and is active
pub async fn exists(
    data: web::Data<AppState>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = Seed::from(response.seed.clone().unwrap());
                let pair = get_pair_from_seed(&user_seed)?;
                let pair_account = pair.public().into_account().to_string();
                let account = sp_core::sr25519::Public::from_str(&pair_account).map_err(map_account_err)?;
                let account = sp_core::crypto::AccountId32::from(account);
                let account_out = account.clone();
                let api = &data.api;
                let result = api.storage().system().account(&account, None).await;
                let data = result.map_err(map_subxt_err)?;
                Ok(HttpResponse::Ok().json(AccountExistsOutput {
                    account: account_out.into(),
                    exists: data.providers > 0,
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in account::fund"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find user::getAttributes"),
            description: format!("Error in account::fund"),
        }))
    }
}

/*
/// Transfers the seed from current user to other account
pub async fn transfer(
    req: web::Json<TransferAccountInput>,
    claims: KeycloakClaims<sugarfunge_api_types::user::ClaimsWithEmail>,
    env: web::Data<Config>
) -> error::Result<HttpResponse> {
    let config = env.clone();
    let env_aux = env.clone();
    match user::get_seed(&claims.sub, env).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                let user_seed = response.seed.clone().unwrap();
                match user::get_sugarfunge_token(env_aux).await {
                    Ok(response) => {                        
                        let awc_client = awc::Client::new();
                        let endpoint = format!("{}/admin/realms/{}/users/{}", config.keycloak_host, config.keycloak_realm, &claims.sub);
                        let attributes = json!({
                            "attributes": ""
                        });
                        let user_response = awc_client.put(endpoint)
                            .append_header((header::ACCEPT, "application/json"), )
                            .append_header((header::CONTENT_TYPE, "application/json"))
                            .append_header((header::AUTHORIZATION, "Bearer ".to_string() + &response.access_token))
                            .send_json(&attributes)
                            .await;            
                        match user_response {
                            Ok(user_response) => {
                                if let StatusCode::NO_CONTENT = user_response.status() {                                    
                                    let awc_client = awc::Client::new();
                                    let endpoint = format!("{}/admin/realms/{}/users/{}", config.keycloak_host, config.keycloak_realm, String::from(&req.to));
                                    let attributes = json!({
                                        "attributes": {
                                            "user-seed": [
                                                user_seed
                                            ]
                                        }
                                    });                        
                                    let receiver_response = awc_client.put(endpoint)
                                        .append_header((header::ACCEPT, "application/json"), )
                                        .append_header((header::CONTENT_TYPE, "application/json"))
                                        .append_header((header::AUTHORIZATION, "Bearer ".to_string() + &response.access_token))
                                        .send_json(&attributes)
                                        .await;
                                    match receiver_response {
                                        Ok(receiver_response) => {
                                            if let StatusCode::NO_CONTENT = receiver_response.status() {
                                                Ok(HttpResponse::Ok().json(TransferAccountOutput {
                                                    message: "Attribute insert to user attributes".to_string()
                                                }))
                                            } 
                                            else {
                                                Ok(HttpResponse::BadRequest().json(RequestError {
                                                    message: json!("Failed to add the Attributes to receiver account"),
                                                    description: format!("Error in account::transfer"),
                                                }))
                                            }
                                        }
                                        Err(_) => {
                                            Ok(HttpResponse::BadRequest().json(RequestError {
                                                message: json!("Failed to add the Attributes to receiver account"),
                                                description: format!("Error in account::transfer"),
                                            }))
                                        }                                        
                                    }                                                                       
                                } else {
                                    Ok(HttpResponse::BadRequest().json(RequestError {
                                        message: json!("Failed to remove seed from account"),
                                        description: format!("Error in account::transfer"),
                                    }))
                                }
                            }                
                            Err(_) => {
                                Ok(HttpResponse::BadRequest().json(RequestError {
                                    message: json!("Not found user Attributes"),
                                    description: format!("Error in account::transfer"),
                                }))
                            }
                        }

                    }
                    Err(_) => {
                        Ok(HttpResponse::BadRequest().json(RequestError {
                            message: json!("Not found user Attributes"),
                            description: format!("Error in account::transfer"),
                        }))
                    }
                }
            } else {
                Ok(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Not found user Attributes"),
                    description: format!("Error in account::transfer"),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Not found user Attributes"),
            description: format!("Error in account::transfer"),
        }))
    }
}
*/