use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::num::TryFromIntError;
use std::result::Result as StdResult;

use candid::{CandidType, Principal};
use ic_cdk::export::candid;
use ic_certified_map::Hash;

#[derive(CandidType, Deserialize, Clone)]
pub struct LogoResult {
    pub logo_type: Cow<'static, str>,
    pub data: Cow<'static, str>,
}

#[derive(CandidType, Deserialize)]
pub struct StableState {
    pub state: State,
    pub hashes: Vec<(String, Hash)>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub logo: Option<LogoResult>,
    pub name: String,
    pub symbol: String,
    pub nfts: Vec<Nft>,
   
}

#[derive(CandidType, Deserialize)]
pub struct Nft {
    pub owner: Principal,
    pub approved: Option<Principal>,
    pub id: u64,
    pub metadata: MetadataDesc,
    pub content: Vec<u8>,
}

pub type MetadataDesc = Vec<MetadataPart>;
pub type MetadataDescRef<'a> = &'a [MetadataPart];

#[derive(CandidType, Deserialize)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: HashMap<String, MetadataVal>,
    pub data: Vec<u8>,
}