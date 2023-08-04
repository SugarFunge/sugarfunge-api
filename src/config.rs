use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub fula_contract_api_host_and_port: String,
    pub labor_token_class_id: u64,
    pub labor_token_asset_id: u64,
    pub labor_token_value: u128,
    pub challenge_token_class_id: u64,
    pub challenge_token_asset_id: u64,
    pub challenge_token_value: u128,
    pub claimed_token_class_id: u64,
    pub claimed_token_asset_id: u64,
}

pub fn init() -> Config {
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        fula_contract_api_host_and_port: match env::var("FULA_CONTRACT_API_HOST_AND_PORT") {
            Ok(var) => var,
            Err(_) => panic!("FULA_CONTRACT_API_HOST_AND_PORT {}", panic_message),
        },
        labor_token_class_id: match env::var("LABOR_TOKEN_CLASS_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("LABOR_TOKEN_CLASS_ID {}", panic_message),
        },
        labor_token_asset_id: match env::var("LABOR_TOKEN_ASSET_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("LABOR_TOKEN_ASSET_ID {}", panic_message),
        },
        challenge_token_class_id: match env::var("CHALLENGE_TOKEN_CLASS_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHALLENGE_TOKEN_CLASS_ID {}", panic_message),
        },
        challenge_token_asset_id: match env::var("CHALLENGE_TOKEN_ASSET_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHALLENGE_TOKEN_ASSET_ID {}", panic_message),
        },
        labor_token_value: match env::var("LABOR_TOKEN_VALUE") {
            Ok(var) => var.parse::<u128>().unwrap(),
            Err(_) => panic!("LABOR_TOKEN_VALUE {}", panic_message),
        },
        challenge_token_value: match env::var("CHALLENGE_TOKEN_VALUE") {
            Ok(var) => var.parse::<u128>().unwrap(),
            Err(_) => panic!("CHALLENGE_TOKEN_VALUE {}", panic_message),
        },
        claimed_token_class_id: match env::var("CLAIMED_TOKEN_CLASS_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CLAIMED_TOKEN_CLASS_ID {}", panic_message),
        },
        claimed_token_asset_id: match env::var("CLAIMED_TOKEN_ASSET_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CLAIMED_TOKEN_ASSET_ID {}", panic_message),
        },
    }
}
