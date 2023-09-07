use std::borrow::Cow;

use candid::Principal;
use ic_cdk::{
    export::{
        candid::{CandidType, Decode, Deserialize, Encode},
    },
};
use ic_stable_structures::{
    BoundedStorable,
    Storable,
};

const ARTICLE_MAX_SIZE: u32 = 2000;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub(crate) struct Article {
    pub article_id: u128,
    pub version: u64,
    pub title: String,
    pub body: String,
}

impl Storable for Article {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Article {
    const MAX_SIZE: u32 = ARTICLE_MAX_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) struct ArticleCreated {
    pub article_id: u128,
    pub version: u64,
    pub title: String,
    pub body: String,
}

