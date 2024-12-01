use serde::{Deserialize, Serialize};

use crate::{
    backend::route::{AuthRoute, Route},
    user::UserId,
};
use http::Method;

use super::{ApiBoth, ApiReq, ApiRes};

//// Register - via Bot Request
pub struct AuthRegister {}

impl ApiBoth for AuthRegister {
    const ROUTE: Route = Route::Auth(AuthRoute::Register);
    const METHOD: Method = Method::POST;

    type Req = AuthRegisterRequest;
    type Res = AuthRegisterResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthRegisterRequest {
    pub omi_uid: String,
    pub tg_uid: i64,
    pub data_check: String,
    pub data_check_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthRegisterResponse {
    pub uid: UserId,
    pub auth_key: String,
}

//// Signin - via Bot Request
pub struct AuthSignin {}

impl ApiBoth for AuthSignin {
    const ROUTE: Route = Route::Auth(AuthRoute::Signin);
    const METHOD: Method = Method::POST;

    type Req = AuthSigninRequest;
    type Res = AuthSigninResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthSigninRequest {
    pub tg_uid: i64,
    pub data_check: String,
    pub data_check_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthSigninResponse {
    pub uid: UserId,
    pub auth_key: String,
}

//// Signout - via User action
pub struct AuthSignout {}

impl ApiReq for AuthSignout {
    const ROUTE: Route = Route::Auth(AuthRoute::Signout);

    const METHOD: Method = Method::POST;

    type Req = AuthSignoutRequest;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthSignoutRequest {
    pub everywhere: bool,
}

//// Check
pub struct AuthCheck {}

impl ApiRes for AuthCheck {
    const ROUTE: Route = Route::Auth(AuthRoute::Check);

    type Res = AuthCheckResponse;

    const METHOD: Method = Method::POST;
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthCheckResponse {
    pub uid: UserId,
}

// Auth token

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AuthTokenKind {
    Signin,
}

impl TryFrom<String> for AuthTokenKind {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "signin" => Ok(Self::Signin),
            _ => Err("invalid kind"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthTokenAfterValidation {
    Delete,
    ExtendExpiresMs(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokenCreateResponse {
    pub id: String,
    pub key: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokenValidateResponse {
    pub uid: UserId,
    pub user_token: String,
}
