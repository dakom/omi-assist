use crate::{
    api_ext::*,
    config::{API_DOMAIN, API_ROOT_PATH},
    telegram::TelegramBot,
    ApiContext,
};
use admin::{
    AdminPopulateFakeUser, AdminPopulateFakeUserRequest, AdminPopulateFakeUserResponse,
    AdminTelegramSetWebHook,
};
use async_trait::async_trait;
use shared::{
    api::*,
    backend::{result::ApiResult, route::Route},
};
use worker::HttpRequest;

use super::auth::register;

#[async_trait(?Send)]
impl ApiEmptyExt for AdminTelegramSetWebHook {
    async fn handle(ctx: &ApiContext<HttpRequest>) -> ApiResult<()> {
        let tg = TelegramBot::new(&ctx.env);

        tg.set_webhook(Route::TelegramWebHook.link(API_DOMAIN, API_ROOT_PATH))
            .await?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl ApiBothExt for AdminPopulateFakeUser {
    type Req = AdminPopulateFakeUserRequest;
    type Res = AdminPopulateFakeUserResponse;

    async fn handle(
        ctx: &ApiContext<AdminPopulateFakeUserRequest>,
    ) -> ApiResult<AdminPopulateFakeUserResponse> {
        let (register, auth_token) = register(&ctx.env, &ctx.req.omi_id, ctx.req.tg_id).await?;

        Ok(AdminPopulateFakeUserResponse {
            register,
            auth_token,
        })
    }
}

impl FromHttpRequest for AdminPopulateFakeUserRequest {}
