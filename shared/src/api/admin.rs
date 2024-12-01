use serde::{Deserialize, Serialize};

use crate::backend::route::{AdminRoute, Route};

use super::{
    auth::{AuthRegisterResponse, AuthTokenCreateResponse},
    ApiBoth, ApiEmpty, Method,
};

pub struct AdminTelegramSetWebHook {}

impl ApiEmpty for AdminTelegramSetWebHook {
    const ROUTE: Route = Route::Admin(AdminRoute::TelegramSetWebHook);
    const METHOD: Method = Method::POST;
}

pub struct AdminPopulateFakeUser {}

impl ApiBoth for AdminPopulateFakeUser {
    const ROUTE: Route = Route::Admin(AdminRoute::PopulateFakeUser);
    const METHOD: Method = Method::POST;

    type Req = AdminPopulateFakeUserRequest;
    type Res = AdminPopulateFakeUserResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminPopulateFakeUserRequest {
    pub omi_id: String,
    pub tg_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminPopulateFakeUserResponse {
    pub register: AuthRegisterResponse,
    pub auth_token: AuthTokenCreateResponse,
}
