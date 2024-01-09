use crate::{config, state::AppState};
use actix_web::{error, web, HttpResponse};
use derive_more::Display;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::ext::sp_core::sr25519::Pair as Sr25519Pair;
use subxt::ext::sp_core::Pair;
use subxt::error::DispatchError;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge::{self};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Display)]
#[display(fmt = "{:?} {:?}", message, description)]
pub struct RequestError {
    pub message: serde_json::Value,
    pub description: String,
}

// Define a struct that represents the health information you want to send.
// Ensure it derives `Serialize`.
#[derive(Serialize)]
struct HealthResponse {
    is_syncing: bool,
    peers: u64,
    should_have_peers: bool,
}

pub fn map_subxt_err(e: subxt::Error) -> actix_web::Error {
    // TODO: json_err should be a json Value to improve UX
    let json_err = json!(e.to_string());
    let req_error = RequestError {
        message: json_err,
        description: "Subxt error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_sf_err(e: subxt::Error) -> actix_web::Error {
    let subxt::Error::Runtime(DispatchError::Module(module_err)) = e else {
        return error::ErrorBadRequest("Not a Module Error");
    };
    let value = module_err.as_root_error::<sugarfunge::Error>().unwrap();
    let mut json_err = json!(&format!("{:?}", value));

    if let Ok(value) = module_err.details() {
        json_err = json!(&format!(
            "Pallet: {}, Variant: {}",
            value.pallet.name(),
            value.variant.name
        ));
    }

    let req_error: RequestError = RequestError {
        message: json_err,
        description: "Sugarfunge error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_account_err(e: sp_core::crypto::PublicError) -> actix_web::Error {
    let json_err: serde_json::Value = json!("Invalid account");
    let req_error = RequestError {
        message: json_err,
        description: format!("{:?}", e),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}
pub fn map_try_from_slice_err(e: std::array::TryFromSliceError) -> actix_web::Error {
    let json_err: serde_json::Value = json!("Invalid slice conversion");
    let req_error = RequestError {
        message: json_err,
        description: format!("{:?}", e),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn get_pair_from_seed(seed: &Seed) -> error::Result<Sr25519Pair> {
    Sr25519Pair::from_string(seed.as_str(), None).map_err(|e| {
        let req_error = RequestError {
            message: json!(&format!("{:?}", e)),
            description: "API error".into(),
        };
        let req_error = serde_json::to_string_pretty(&req_error).unwrap();
        error::ErrorBadRequest(req_error)
    })
}

pub fn url_to_string(url: Url) -> String {
    let mut res = url.to_string();
    match (url.port(), url.port_or_known_default()) {
        (None, Some(port)) => {
            res.insert_str(res.len() - 1, &format!(":{}", port));
            res
        }
        _ => res,
    }
}

pub fn map_fula_err(e: subxt::Error) -> actix_web::Error {
    let subxt::Error::Runtime(DispatchError::Module(module_err)) = e else {
        return error::ErrorBadRequest("Not a Module Error");
    };
    let value = module_err.as_root_error::<sugarfunge::Error>().unwrap();
    let mut json_err = json!(&format!("{:?}", value));

    if let Ok(value) = module_err.details() {
        json_err = json!(&format!(
            "Pallet: {}, Variant: {}",
            value.pallet.name(),
            value.variant.name
        ));
    }

    let req_error: RequestError = RequestError {
        message: json_err,
        description: "Fula Pallet error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}
pub fn map_fula_pool_err(e: subxt::Error) -> actix_web::Error {
    let subxt::Error::Runtime(DispatchError::Module(module_err)) = e else {
        return error::ErrorBadRequest("Not a Module Error");
    };
    let value = module_err.as_root_error::<sugarfunge::Error>().unwrap();
    let mut json_err = json!(&format!("{:?}", value));

    if let Ok(value) = module_err.details() {
        json_err = json!(&format!(
            "Pallet: {}, Variant: {}",
            value.pallet.name(),
            value.variant.name
        ));
    }

    let req_error: RequestError = RequestError {
        message: json_err,
        description: "Fula Pool error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub async fn health_check(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let rpc = &data.rpc;
    let health = rpc.system_health().await.map_err(map_subxt_err)?;

    // Map the fields from SystemHealth to HealthResponse
    let health_response = HealthResponse {
        is_syncing: health.is_syncing,
        peers: health.peers as u64,
        should_have_peers: health.should_have_peers,
    };

    // Respond with the serialized HealthResponse
    Ok(HttpResponse::Ok().json(health_response))
}

// Function to get the hash using the blake2_256 of a [u8] value
pub fn hash(s: &[u8]) -> sp_core::H256 {
    sp_io::hashing::blake2_256(s).into()
}

// Function to build the endpoints routes when executed the req function
fn endpoint(cmd: &'static str) -> String {
    dotenv().ok();
    let env = config::init();
    format!("{}/{}", env.fula_contract_api_host_and_port.as_str(), cmd)
}

// Function to create a request to the fula-contract-api, given the endpoint route and the inputs
pub async fn request<'a, I, O>(cmd: &'static str, args: I) -> Result<O, RequestError>
where
    I: Serialize,
    O: for<'de> Deserialize<'de>,
{
    let sf_res = reqwest::Client::new()
        .post(endpoint(cmd))
        .json(&args)
        .send()
        .await;

    match sf_res {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match res.json::<RequestError>().await {
                    Ok(err) => Err(err),
                    Err(_) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            } else {
                match res.json().await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            }
        }
        Err(err) => Err(RequestError {
            message: json!(format!("{:#?}", err)),
            description: "Reqwest error.".into(),
        }),
    }
}
