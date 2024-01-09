use std::{ops::Div, str::FromStr};

use serde::{Deserialize, Serialize};

use sp_core;

use sp_core::U256;

use bevy_derive::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct Seed(String);

impl From<String> for Seed {
    fn from(seed: String) -> Seed {
        Seed(seed)
    }
}

impl From<&Seed> for String {
    fn from(seed: &Seed) -> String {
        seed.0.clone()
    }
}

impl Seed {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct Account(String);

impl From<String> for Account {
    fn from(account: String) -> Account {
        Account(account)
    }
}

impl From<sp_core::crypto::AccountId32> for Account {
    fn from(account: sp_core::crypto::AccountId32) -> Account {
        Account(account.to_string())
    }
}

impl From<subxt::utils::AccountId32> for Account {
    fn from(account: subxt::utils::AccountId32) -> Account {
        Account(account.to_string())
    }
}

impl TryFrom<&Account> for sp_core::crypto::AccountId32 {
    type Error = sp_core::crypto::PublicError;

    fn try_from(
        account: &Account,
    ) -> Result<sp_core::crypto::AccountId32, sp_core::crypto::PublicError> {
        let account = sp_core::sr25519::Public::from_str(account.as_str());
        match account {
            Ok(account) => Ok(sp_core::crypto::AccountId32::from(account)),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<&Account> for subxt::utils::AccountId32 {
    type Error = sp_core::crypto::PublicError;

    fn try_from(
        account: &Account,
    ) -> Result<subxt::utils::AccountId32, sp_core::crypto::PublicError> {
        let account = sp_core::sr25519::Public::from_str(account.as_str());
        match account {
            Ok(account) => Ok(subxt::utils::AccountId32::from(
                account.as_array_ref().to_owned(),
            )),
            Err(err) => Err(err),
        }
    }
}

impl From<&Account> for String {
    fn from(account: &Account) -> String {
        account.0.clone()
    }
}

impl Account {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct MarketId(u64);

impl From<u64> for MarketId {
    fn from(id: u64) -> MarketId {
        MarketId(id)
    }
}

impl From<MarketId> for u64 {
    fn from(id: MarketId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct ClassId(u64);

impl From<u64> for ClassId {
    fn from(id: u64) -> ClassId {
        ClassId(id)
    }
}

impl From<ClassId> for u64 {
    fn from(id: ClassId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct AssetId(u64);

impl From<u64> for AssetId {
    fn from(id: u64) -> AssetId {
        AssetId(id)
    }
}

impl From<AssetId> for u64 {
    fn from(id: AssetId) -> u64 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct Balance(u128);

impl From<u128> for Balance {
    fn from(id: u128) -> Balance {
        Balance(id)
    }
}

impl From<Balance> for u128 {
    fn from(id: Balance) -> u128 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct Amount(i128);

impl From<i128> for Amount {
    fn from(id: i128) -> Amount {
        Amount(id)
    }
}

impl From<Amount> for i128 {
    fn from(id: Amount) -> i128 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct BundleId(String);

impl From<String> for BundleId {
    fn from(bundleid: String) -> BundleId {
        BundleId(bundleid)
    }
}

impl From<&BundleId> for String {
    fn from(bundleid: &BundleId) -> String {
        bundleid.0.clone()
    }
}

impl BundleId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_u64(&self) -> Result<u64, std::num::ParseIntError> {
        self.0.parse::<u64>()
    }
}

impl FromIterator<char> for BundleId {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> BundleId {
        let mut buf = String::new();
        buf.extend(iter);
        BundleId::from(buf)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct ValidatorId(String);

impl From<String> for ValidatorId {
    fn from(validatorid: String) -> ValidatorId {
        ValidatorId(validatorid)
    }
}

impl From<&ValidatorId> for String {
    fn from(validatorid: &ValidatorId) -> String {
        validatorid.0.clone()
    }
}

impl ValidatorId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct Cid(String);
impl From<String> for Cid {
    fn from(cid: String) -> Cid {
        Cid(cid.clone())
    }
}
impl From<&Cid> for String {
    fn from(cid: &Cid) -> String {
        cid.0.clone()
    }
}
impl Cid {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct Name(String);
impl From<String> for Name {
    fn from(name: String) -> Name {
        Name(name.clone())
    }
}
impl From<&Name> for String {
    fn from(name: &Name) -> String {
        name.0.clone()
    }
}
impl Name {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut)]
pub struct PeerId(String);
impl From<String> for PeerId {
    fn from(peer_id: String) -> PeerId {
        PeerId(peer_id.clone())
    }
}
impl From<&PeerId> for String {
    fn from(peer_id: &PeerId) -> String {
        peer_id.0.clone()
    }
}
impl PeerId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct PoolId(u32);
impl From<u32> for PoolId {
    fn from(id: u32) -> PoolId {
        PoolId(id)
    }
}
impl From<PoolId> for u32 {
    fn from(id: PoolId) -> u32 {
        id.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Deref, DerefMut)]
pub struct ReplicationFactor(u16);
impl From<u16> for ReplicationFactor {
    fn from(id: u16) -> ReplicationFactor {
        ReplicationFactor(id)
    }
}
impl From<ReplicationFactor> for u16 {
    fn from(id: ReplicationFactor) -> u16 {
        id.0
    }
}

pub fn transform_vec_account_to_string(in_vec: Vec<Account>) -> Vec<String> {
    in_vec
        .into_iter()
        .map(|account| String::from(&account))
        .collect()
}

pub fn transform_vec_string_to_account(in_vec: Vec<String>) -> Vec<Account> {
    in_vec.into_iter().map(Account::from).collect()
}

pub fn transform_vec_balance_to_u128(in_vec: &[Balance]) -> Vec<u128> {
    in_vec.iter().map(|balance| u128::from(*balance)).collect()
}

pub fn transform_vec_classid_to_u64(in_vec: Vec<ClassId>) -> Vec<u64> {
    in_vec.into_iter().map(u64::from).collect()
}

pub fn transform_vec_assetid_to_u64(in_vec: Vec<AssetId>) -> Vec<u64> {
    in_vec.into_iter().map(u64::from).collect()
}

pub fn transform_doublevec_assetid_to_u64(in_vec: Vec<Vec<AssetId>>) -> Vec<Vec<u64>> {
    in_vec
        .into_iter()
        .map(|assetid| assetid.into_iter().map(u64::from).collect())
        .collect()
}

pub fn transform_doublevec_balance_to_u128(in_vec: Vec<Vec<Balance>>) -> Vec<Vec<u128>> {
    in_vec
        .into_iter()
        .map(|balance| balance.into_iter().map(u128::from).collect())
        .collect()
}

pub fn transform_option_account_value(value: Option<subxt::utils::AccountId32>) -> Option<Account> {
    if let Some(value) = value {
        return Some(value.into());
    }
    return None::<Account>;
}
pub fn transform_option_pool_value(value: Option<u32>) -> Option<PoolId> {
    if let Some(value) = value {
        return Some(value.into());
    }
    return None::<PoolId>;
}
pub fn transform_storage_output(storers: Vec<subxt::utils::AccountId32>) -> Vec<String> {
    storers
        .into_iter()
        .map(|current_storer| current_storer.to_string())
        .collect()
}
pub fn remove_decimals_from_u256(value: U256, decimals: u32) -> u128 {
    return value.div(10_u128.pow(decimals) as u128).as_u128();
}
