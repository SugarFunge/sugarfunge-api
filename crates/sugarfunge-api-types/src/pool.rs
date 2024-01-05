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
    // Africa (Cape Town) - Covering Cape Town, Durban, Pretoria.
    AfricaCapeTown,
    // Asia Pacific (Hong Kong) - Covering Hong Kong, Macau.
    AsiaPacificHongKong,
    // Asia Pacific (Hyderabad) - Covering Hyderabad, Bangalore, Chennai.
    AsiaPacificHyderabad,
    // Asia Pacific (Jakarta) - Covering Jakarta, Surabaya, Bandung.
    AsiaPacificJakarta,
    // Asia Pacific (Melbourne) - Covering Melbourne, Sydney, Brisbane.
    AsiaPacificMelbourne,
    // Canada (Calgary) - Covering Calgary, Edmonton, Vancouver.
    CanadaCalgary,
    // Europe (Zurich) - Covering Zurich, Geneva, Basel.
    EuropeZurich,
    // Europe (Milan) - Covering Milan, Rome, Naples.
    EuropeMilan,
    // Europe (Spain) - Covering Madrid, Barcelona, Valencia.
    EuropeSpain,
    // Israel (Tel Aviv) - Covering Tel Aviv, Jerusalem, Haifa.
    IsraelTelAviv,
    // Middle East (UAE) - Covering Dubai, Abu Dhabi, Sharjah.
    MiddleEastUAE,
    // Middle East (Bahrain) - Covering Manama, Riffa, Muharraq.
    MiddleEastBahrain,
    // Asia Pacific (Tokyo) - Covering Tokyo, Yokohama, Osaka.
    AsiaPacificTokyo,
    // Asia Pacific (Seoul) - Covering Seoul, Incheon, Busan.
    AsiaPacificSeoul,
    // Asia Pacific (Osaka) - Covering Osaka, Kyoto, Kobe.
    AsiaPacificOsaka,
    // Asia Pacific (Mumbai) - Covering Mumbai, Pune, Nagpur.
    AsiaPacificMumbai,
    // Asia Pacific (Singapore) - Covering Singapore, Johor Bahru, Batam.
    AsiaPacificSingapore,
    // Asia Pacific (Sydney) - Covering Sydney, Canberra, Newcastle.
    AsiaPacificSydney,
    // Canada (Central) - Covering Toronto, Ottawa, Montreal.
    CanadaCentral,
    // Europe (Frankfurt) - Covering Frankfurt, Munich, Stuttgart.
    EuropeFrankfurt,
    // Europe (Stockholm) - Covering Stockholm, Gothenburg, Malmö.
    EuropeStockholm,
    // Europe (Ireland) - Covering Dublin, Cork, Limerick.
    EuropeIreland,
    // Europe (London) - Covering London, Birmingham, Manchester.
    EuropeLondon,
    // Europe (Paris) - Covering Paris, Lyon, Marseille.
    EuropeParis,
    // South America (São Paulo) - Covering São Paulo, Rio de Janeiro, Belo Horizonte.
    SouthAmericaSaoPaulo,
    // US East (N. Virginia) - Covering Washington D.C., Baltimore, Richmond.
    UsEastNVirginia,
    // US East (Ohio) - Covering Columbus, Cleveland, Cincinnati.
    UsEastOhio,
    // US West (N. California) - Covering San Francisco, San Jose, Oakland.
    UsWestNCalifornia,
    // US West (Oregon) - Covering Portland, Eugene, Salem.
    UsWestOregon,
    // ... [other regions can be added similarly]
    Other,
}

impl Into<Region> for &String {
    fn into(self) -> Region {
        match self.as_str() {
            "AfricaCapeTown" => Region::AfricaCapeTown,
            "AsiaPacificHongKong" => Region::AsiaPacificHongKong,
            "AsiaPacificHyderabad" => Region::AsiaPacificHyderabad,
            "AsiaPacificJakarta" => Region::AsiaPacificJakarta,
            "AsiaPacificMelbourne" => Region::AsiaPacificMelbourne,
            "CanadaCalgary" => Region::CanadaCalgary,
            "EuropeZurich" => Region::EuropeZurich,
            "EuropeMilan" => Region::EuropeMilan,
            "EuropeSpain" => Region::EuropeSpain,
            "IsraelTelAviv" => Region::IsraelTelAviv,
            "MiddleEastUAE" => Region::MiddleEastUAE,
            "MiddleEastBahrain" => Region::MiddleEastBahrain,
            "AsiaPacificTokyo" => Region::AsiaPacificTokyo,
            "AsiaPacificSeoul" => Region::AsiaPacificSeoul,
            "AsiaPacificOsaka" => Region::AsiaPacificOsaka,
            "AsiaPacificMumbai" => Region::AsiaPacificMumbai,
            "AsiaPacificSingapore" => Region::AsiaPacificSingapore,
            "AsiaPacificSydney" => Region::AsiaPacificSydney,
            "CanadaCentral" => Region::CanadaCentral,
            "EuropeFrankfurt" => Region::EuropeFrankfurt,
            "EuropeStockholm" => Region::EuropeStockholm,
            "EuropeIreland" => Region::EuropeIreland,
            "EuropeLondon" => Region::EuropeLondon,
            "EuropeParis" => Region::EuropeParis,
            "SouthAmericaSaoPaulo" => Region::SouthAmericaSaoPaulo,
            "UsEastNVirginia" => Region::UsEastNVirginia,
            "UsEastOhio" => Region::UsEastOhio,
            "UsWestNCalifornia" => Region::UsWestNCalifornia,
            "UsWestOregon" => Region::UsWestOregon,
            _ => Region::Other,
        }
    }
}

impl Into<Vec<u8>> for Region {
    fn into(self) -> Vec<u8> {
        match self {
            Region::AfricaCapeTown => "AfricaCapeTown".as_bytes().to_vec(),
            Region::AsiaPacificHongKong => "AsiaPacificHongKong".as_bytes().to_vec(),
            Region::AsiaPacificHyderabad => "AsiaPacificHyderabad".as_bytes().to_vec(),
            Region::AsiaPacificJakarta => "AsiaPacificJakarta".as_bytes().to_vec(),
            Region::AsiaPacificMelbourne => "AsiaPacificMelbourne".as_bytes().to_vec(),
            Region::CanadaCalgary => "CanadaCalgary".as_bytes().to_vec(),
            Region::EuropeZurich => "EuropeZurich".as_bytes().to_vec(),
            Region::EuropeMilan => "EuropeMilan".as_bytes().to_vec(),
            Region::EuropeSpain => "EuropeSpain".as_bytes().to_vec(),
            Region::IsraelTelAviv => "IsraelTelAviv".as_bytes().to_vec(),
            Region::MiddleEastUAE => "MiddleEastUAE".as_bytes().to_vec(),
            Region::MiddleEastBahrain => "MiddleEastBahrain".as_bytes().to_vec(),
            Region::AsiaPacificTokyo => "AsiaPacificTokyo".as_bytes().to_vec(),
            Region::AsiaPacificSeoul => "AsiaPacificSeoul".as_bytes().to_vec(),
            Region::AsiaPacificOsaka => "AsiaPacificOsaka".as_bytes().to_vec(),
            Region::AsiaPacificMumbai => "AsiaPacificMumbai".as_bytes().to_vec(),
            Region::AsiaPacificSingapore => "AsiaPacificSingapore".as_bytes().to_vec(),
            Region::AsiaPacificSydney => "AsiaPacificSydney".as_bytes().to_vec(),
            Region::CanadaCentral => "CanadaCentral".as_bytes().to_vec(),
            Region::EuropeFrankfurt => "EuropeFrankfurt".as_bytes().to_vec(),
            Region::EuropeStockholm => "EuropeStockholm".as_bytes().to_vec(),
            Region::EuropeIreland => "EuropeIreland".as_bytes().to_vec(),
            Region::EuropeLondon => "EuropeLondon".as_bytes().to_vec(),
            Region::EuropeParis => "EuropeParis".as_bytes().to_vec(),
            Region::SouthAmericaSaoPaulo => "SouthAmericaSaoPaulo".as_bytes().to_vec(),
            Region::UsEastNVirginia => "UsEastNVirginia".as_bytes().to_vec(),
            Region::UsEastOhio => "UsEastOhio".as_bytes().to_vec(),
            Region::UsWestNCalifornia => "UsWestNCalifornia".as_bytes().to_vec(),
            Region::UsWestOregon => "UsWestOregon".as_bytes().to_vec(),
            Region::Other => "Other".as_bytes().to_vec(),
        }
    }
}

