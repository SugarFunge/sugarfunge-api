use crate::account;
use crate::util::*;
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    body,
    error,
    http::{header, StatusCode}, 
};
use actix_web_middleware_keycloak_auth::{KeycloakClaims};
use awc::{self};
use serde_json::json;
use sugarfunge_api_types::account::*;
use sugarfunge_api_types::config::Config;
use sugarfunge_api_types::user::*;

/// Generate admin token for sugarfunge-api client
pub async fn get_sugarfunge_token(env: web::Data<Config>) -> error::Result<SugarTokenOutput, HttpResponse> {
    let config = &env;
    let endpoint = config.keycloak_host.to_string() + "/auth/realms/" + &config.keycloak_realm + "/protocol/openid-connect/token";

    let credentials = web::Data::new(Credentials{
        client_id: config.keycloak_client_id.to_string(),
        grant_type: "password".to_string(),
        username: config.keycloak_username.to_owned(),
        password: config.keycloak_user_password.to_owned(),
        client_secret: config.keycloak_client_secret.to_owned(),
        scope: "openid".to_string()
    });

    let awc_client = awc::Client::new();

    let response = awc_client.post(endpoint)
        .insert_header((header::CONTENT_TYPE, "application/x-www-form-urlencoded"))
        .send_form(&credentials)
        .await; 

    match response {
        Ok(mut response) => {
            match response.status() {
                StatusCode::OK => {
                    let body_str: String = std::str::from_utf8(&response.body().await.unwrap()).unwrap().to_string();
                    let body: SugarTokenOutput = serde_json::from_str(&body_str).unwrap();
                    Ok(body)
                },
                _ => {
                    Err(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to get the sugarfunge token"),
                        description: format!("Error in user::getSugarfungeToken"),
                    }))
                }
            }
        },
        Err(_) => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to connect to keycloak"),
            description: format!("Error in user::getSugarfungeToken"),
        }))
    }
}

/// Checks if the user has a seed attribute
pub async fn get_seed(
    user_id: &String,
    env: web::Data<Config>
) -> error::Result<web::Json<UserSeedOutput>, HttpResponse> { 
    let config = env.clone();

    match get_sugarfunge_token(env).await {
        Ok(response) => {
            let awc_client = awc::Client::new();
            let endpoint = format!("{}/auth/admin/realms/{}/users/{}", config.keycloak_host, config.keycloak_realm, user_id); 

            let user_response = awc_client.get(endpoint)
                .append_header((header::ACCEPT, "application/json"), )
                .append_header((header::CONTENT_TYPE, "application/json"))
                .append_header((header::AUTHORIZATION, "Bearer ".to_string() + &response.access_token))
                .send()
                .await; 

            match user_response {
                Ok(mut user_response) => {                    
                    match user_response.status() {
                        StatusCode::OK => {
                            let body_str: String = std::str::from_utf8(&user_response.body().await.unwrap()).unwrap().to_string();
                            let user_info: UserInfo = serde_json::from_str(&body_str).unwrap();

                            if !user_info.attributes.clone().unwrap_or_default().user_seed.is_empty() {
                                let user_seed = user_info.attributes.clone().unwrap_or_default().user_seed[0].clone();
                                Ok(web::Json(UserSeedOutput {
                                    seed: Some(user_seed)
                                }))
                            } else {
                                Ok(web::Json(UserSeedOutput {
                                    seed: Some("".to_string())
                                }))
                            }
                        },
                        _ => Err(HttpResponse::BadRequest().json(RequestError {
                                message: json!("Failed to get the attributes"),
                                description: format!("Error in user::getSeed"),
                            }))
                    }
                }
                Err(_) => Err(HttpResponse::BadRequest().json(RequestError {
                    message: json!("Failed to get the attributes"),
                    description: format!("Error in user::getSeed"),
                }))
            }
        }
        Err(_e) => Err(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to get the sugarfunge token"),
            description: format!("Error in user::getSugarfungeToken"),
        }))
    }
}

/// Inserts a seed attribute to the user
pub async fn insert_seed(
    user_id: &String,
    req: HttpRequest,
    env: web::Data<Config>
) -> error::Result<web::Json<InsertUserSeedOutput>, HttpResponse> { 
    let config = env.clone();

    match get_sugarfunge_token(env).await {
        Ok(response) => {
            match account::create(req).await {
                Ok(response_account) => {                    
                    
                    let bytes = body::to_bytes(response_account.into_body()).await.unwrap();
                    let str_bytes = std::str::from_utf8(&bytes).unwrap().to_string();
                    let body: CreateAccountOutput = serde_json::from_str(&str_bytes).unwrap();
                    
                    let awc_client = awc::Client::new();
                    let endpoint = format!("{}/auth/admin/realms/{}/users/{}", config.keycloak_host, config.keycloak_realm, user_id); 

                    let attributes = json!({
                        "attributes": {
                            "user-seed": [
                                body.seed
                            ]
                        }
                    });
        
                    let response = awc_client.put(endpoint)
                        .append_header((header::ACCEPT, "application/json"), )
                        .append_header((header::CONTENT_TYPE, "application/json"))
                        .append_header((header::AUTHORIZATION, "Bearer ".to_string() + &response.access_token))
                        .send_json(&attributes)
                        .await;
        
                    match response {
                        Ok(response) => {
                            match response.status() { 
                                StatusCode::NO_CONTENT => {
                                    Ok(web::Json(InsertUserSeedOutput {
                                        message: "User had no Seed, Seed inserted to user attributes".to_string()
                                    }))
                                }
                                _ => {
                                    Err(HttpResponse::BadRequest().json(RequestError {
                                        message: json!("Failed to insert seed"),
                                        description: format!("Error in user::insertSeedUser"),
                                    }))
                                }
                            }
                        }
                        Err(_e) => {
                            Err(HttpResponse::BadRequest().json(RequestError {
                                message: json!("Failed to get the sugarfunge token"),
                                description: format!("Error in user::getSugarfungeToken"),
                            }))
                        }
                    }
                }
                Err(_) => {
                    Err(HttpResponse::BadRequest().json(RequestError {
                        message: json!("Failed to create account"),
                        description: format!("Error in account::create"),
                    }))
                }
            }            

        }
        Err(_error) => {
            Err(HttpResponse::BadRequest().json(RequestError {
                message: json!("Failed to get the sugarfunge token"),
                description: format!("Error in user::getSugarfungeToken"),
            }))
        }
    }
}

/// Check if user has a seed. If he doesn't, it is created and added
pub async fn verify_seed(
    claims: KeycloakClaims<ClaimsWithEmail>,
    req: HttpRequest,
    env: web::Data<Config>
) ->  error::Result<HttpResponse> {
    match get_seed(&claims.sub, env.clone()).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                Ok(HttpResponse::Ok().json(InsertUserSeedOutput {
                    message: "User already has a seed".to_string()
                }))
            } else {
                match insert_seed(&claims.sub, req, env).await {
                    Ok(response) => {Ok(HttpResponse::Ok().json(response))}
                    Err(_) => {
                        Ok(HttpResponse::BadRequest().json(RequestError {
                            message: json!("Failed to insert seed"),
                            description: format!("Error in user::insertSeedUser"),
                        }))
                    }
                }
            }
        },
        Err(_) => {
            Ok(HttpResponse::BadRequest().json(RequestError {
                message: json!("Failed to connect to keycloak"),
                description: format!("Error in user::verifySeed"),
            }))
        }
    }
}