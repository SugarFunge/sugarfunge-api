use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub keycloak_public_key: String,
    pub keycloak_client_id: String,
    pub keycloak_username: String,
    pub keycloak_user_password: String,
    pub keycloak_client_secret: String,
    pub keycloak_host: String,
    pub keycloak_realm: String
}

pub fn init() -> Config {
 
    let panic_message: String = "environment variable is not set".to_string();

    Config {
        keycloak_public_key: match env::var("KEYCLOAK_PUBLIC_KEY") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_PUBLIC_KEY {}", panic_message)
        },
        keycloak_client_id: match env::var("KEYCLOAK_CLIENT_ID") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_CLIENT_ID {}", panic_message)
        },
        keycloak_username: match env::var("KEYCLOAK_USERNAME") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_USERNAME {}", panic_message)
        },
        keycloak_user_password: match env::var("KEYCLOAK_USER_PASSWORD") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_USER_PASSWORD {}", panic_message)
        },
        keycloak_client_secret: match env::var("KEYCLOAK_CLIENT_SECRET") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_CLIENT_SECRET {}", panic_message)
        },
        keycloak_host: match env::var("KEYCLOAK_HOST") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_HOST {}", panic_message)
        },
        keycloak_realm: match env::var("KEYCLOAK_REALM") {
            Ok(var) => var,
            Err(_) => panic!("KEYCLOAK_REALM {}", panic_message)
        }
    }
}