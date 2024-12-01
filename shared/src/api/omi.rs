use http::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::backend::route::Route;

use super::ApiReq;

pub struct OmiWebHook {}

impl ApiReq for OmiWebHook {
    const ROUTE: Route = Route::OmiWebHook;
    const METHOD: Method = Method::POST;

    type Req = OmiWebHookRequest;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OmiWebHookRequest {
    pub omi_uid: String,
    pub payload: OmiPayload,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OmiPayload {
    pub segments: Vec<OmiSegment>,
    pub session_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OmiSegment {
    pub text: String,
    pub speaker: Option<String>,
    pub speaker_id: Option<i32>,
    pub is_user: Option<bool>,
    pub person_id: Option<i32>,
    pub start: Option<f64>,
    pub end: Option<f64>,
}

#[derive(Serialize, Deserialize, Error, Debug, Clone, PartialEq)]
pub enum OmiHookError {
    #[error("No such user: {0}")]
    NoSuchUser(String),

    #[error("No matching action for user: {0}")]
    NoActions(String),
}
