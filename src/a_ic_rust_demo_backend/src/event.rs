use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
    },
};
use ic_stable_structures::Storable;

use crate::article::ArticleCreated;
use crate::article::ArticleUpdated;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) enum Event {
    ArticleEvent(ArticleEvent),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) enum ArticleEvent {
    ArticleCreated(ArticleCreated),
    ArticleUpdated(ArticleUpdated),
}


impl Storable for Event {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
