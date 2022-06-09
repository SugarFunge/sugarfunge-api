use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub grant_type: String,
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub client_secret: String,
    pub scope: String
}

#[derive(Serialize, Deserialize)]
pub struct SugarTokenOutput {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessageOutput {
    pub error: String,
    pub message: String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub attributes: Option<UserAtributes>,
    pub email: String,
    #[serde(rename = "emailVerified", default)]
    pub email_verified: bool,
    pub username: String,
    #[serde(rename = "firstName", default)]
    pub first_name: String,
    #[serde(rename = "lastName", default)]
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSeedOutput {
    pub seed: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUserSeedOutput {
    pub error: Option<String>,
    pub message: String
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ClaimsWithEmail {
    pub sub: String,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String
}