use crate::primitives::*;
use serde::{Deserialize, Serialize};

// CREATE POOL

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePoolInput {
    pub seed: Seed,
    pub pool_name: Name,
    pub peer_id: PeerId,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePoolOutput {
    pub owner: Option<Account>,
    pub pool_id: PoolId,
}

// LEAVE POOL

#[derive(Serialize, Deserialize, Debug)]
pub struct LeavePoolInput {
    pub seed: Seed,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeavePoolOutput {
    pub pool_id: PoolId,
    pub account: Account,
}

// JOIN POOL

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinPoolInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub peer_id: PeerId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinPoolOutput {
    pub pool_id: PoolId,
    pub account: Account,
}

// CANCEL JOIN POOL

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelJoinPoolInput {
    pub seed: Seed,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelJoinPoolOutput {
    pub pool_id: PoolId,
    pub account: Account,
}

// VOTE

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub account: Account,
    pub vote_value: bool,
    pub peer_id: PeerId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteOutput {
    pub pool_id: PoolId,
    pub account: Account,
    pub result: String,
}

// GET POOLS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolInput {
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolsOutput {
    pub pools: Vec<PoolData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolData {
    pub pool_id: PoolId,
    pub creator: Option<Account>,
    pub pool_name: Name,
    pub region: String,
    pub parent: Option<PoolId>,
    pub participants: Vec<Account>,
}

// GET USERS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolUsersInput {
    pub account: Option<Account>,
    pub pool_id: Option<PoolId>,
    pub request_pool_id: Option<PoolId>, // New field added for additional filter
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolUsersOutput {
    pub users: Vec<PoolUserData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolUserData {
    pub account: Account,
    pub pool_id: Option<PoolId>,
    pub request_pool_id: Option<PoolId>,
    pub peer_id: PeerId,
}

// GET POOLREQUESTS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolRequestInput {
    pub pool_id: Option<PoolId>,
    pub account: Option<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllPoolRequestsOutput {
    pub poolrequests: Vec<PoolRequestData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolRequestData {
    pub pool_id: PoolId,
    pub account: Account,
    pub voted: Vec<Account>,
    pub positive_votes: u16,
    pub peer_id: PeerId,
}

// FUNCTIONS TO MANAGE THE REGIONS

#[derive(Serialize, Deserialize, Debug)]
/// An enum that represents the region of the pool
pub enum Region {
    Alberta,
    BritishColumbia,
    Manitoba,
    NewBrunswick,
    NewfoundlandAndLabrador,
    NovaScotia,
    Ontario,
    PrinceEdwardIsland,
    Quebec,
    Saskatchewan,
    NorthwestTerritories,
    Nunavut,
    Yukon,
    Other,
}

impl Into<Region> for &String {
    fn into(self) -> Region {
        match self.as_str() {
            "Alberta" => Region::Alberta,
            "BritishColumbia" => Region::BritishColumbia,
            "Manitoba" => Region::Manitoba,
            "NewBrunswick" => Region::NewBrunswick,
            "NewfoundlandAndLabrador" => Region::NewfoundlandAndLabrador,
            "NovaScotia" => Region::NovaScotia,
            "Ontario" => Region::Ontario,
            "PrinceEdwardIsland" => Region::PrinceEdwardIsland,
            "Quebec" => Region::Quebec,
            "Saskatchewan" => Region::Saskatchewan,
            "NorthwestTerritories" => Region::NorthwestTerritories,
            "Nunavut" => Region::Nunavut,
            "Yukon" => Region::Yukon,
            _ => Region::Other,
        }
    }
}

impl Into<Vec<u8>> for Region {
    fn into(self) -> Vec<u8> {
        match self {
            Region::Alberta => "Alberta".as_bytes().to_vec(),
            Region::BritishColumbia => "BritishColumbia".as_bytes().to_vec(),
            Region::Manitoba => "Manitoba".as_bytes().to_vec(),
            Region::NewBrunswick => "NewBrunswick".as_bytes().to_vec(),
            Region::NewfoundlandAndLabrador => "NewfoundlandAndLabrador".as_bytes().to_vec(),
            Region::NovaScotia => "NovaScotia".as_bytes().to_vec(),
            Region::Ontario => "Ontario".as_bytes().to_vec(),
            Region::PrinceEdwardIsland => "PrinceEdwardIsland".as_bytes().to_vec(),
            Region::Quebec => "Quebec".as_bytes().to_vec(),
            Region::Saskatchewan => "Saskatchewan".as_bytes().to_vec(),
            Region::NorthwestTerritories => "NorthwestTerritories".as_bytes().to_vec(),
            Region::Nunavut => "Nunavut".as_bytes().to_vec(),
            Region::Yukon => "Yukon".as_bytes().to_vec(),
            Region::Other => "Other".as_bytes().to_vec(),
        }
    }
}
