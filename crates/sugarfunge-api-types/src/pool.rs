use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePoolInput {
    pub seed: Seed,
    pub pool_name: String,
    pub peer_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePoolOutput {
    pub owner: Option<Account>,
    pub pool_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeavePoolInput {
    pub seed: Seed,
    pub pool_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeavePoolOutput {
    pub pool_id: u32,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinPoolInput {
    pub seed: Seed,
    pub pool_id: u32,
    pub peer_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinPoolOutput {
    pub pool_id: u32,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelJoinPoolInput {
    pub seed: Seed,
    pub pool_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelJoinPoolOutput {
    pub pool_id: u32,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteInput {
    pub seed: Seed,
    pub pool_id: u32,
    pub account: Account,
    pub vote_value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteOutput {
    pub pool_id: u32,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolData {
    pub pool_id: u32,
    pub owner: Option<Account>,
    pub pool_name: String,
    pub parent: Option<u32>,
    pub participants: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolsOutput {
    pub pools: Vec<PoolData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolUserData {
    pub account: Account,
    pub pool_id: Option<u32>,
    pub request_pool_id: Option<u32>,
    pub peer_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolUsersInput {
    pub account: Option<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolUsersOutput {
    pub users: Vec<PoolUserData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolRequestData {
    pub pool_id: u32,
    pub account: Account,
    pub voted: Vec<Account>,
    pub positive_votes: u16,
    pub peer_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolRequestsOutput {
    pub poolrequests: Vec<PoolRequestData>,
}
