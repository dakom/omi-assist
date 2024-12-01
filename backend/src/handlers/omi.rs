use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use shared::api::{
    omi::{OmiHookError, OmiPayload, OmiWebHook, OmiWebHookRequest},
    ApiReq,
};

use crate::{
    api_ext::{ApiReqExt, FromHttpRequest},
    db::{
        action::TelegramActionDb,
        user::{OmiAccount, TelegramAccount},
    },
    prelude::*,
    telegram::TelegramBot,
};

#[async_trait(?Send)]
impl ApiReqExt for OmiWebHook {
    type Req = <Self as ApiReq>::Req;

    async fn handle(ctx: &ApiContext<OmiWebHookRequest>) -> ApiResult<()> {
        if ctx.req.payload.segments.len() == 0 {
            return Ok(());
        }

        let (actions_to_send, user_id) = match OmiAccount::load(&ctx.env, &ctx.req.omi_uid).await {
            Ok(omi_account) => {
                let user_id = omi_account.user_id.clone();
                let mut actions_to_send = Vec::new();
                match TelegramActionDb::list(&ctx.env, &omi_account.user_id).await {
                    Ok(actions) => {
                        for action in actions {
                            let hit = ctx.req.payload.segments.iter().any(|segment| {
                                segment
                                    .text
                                    .to_lowercase()
                                    .contains(&action.prompt.to_lowercase())
                            });
                            if hit {
                                actions_to_send.push(action);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(ApiError::Omi(OmiHookError::NoActions(
                            ctx.req.omi_uid.clone(),
                        )));
                    }
                }

                (actions_to_send, user_id)
            }
            Err(_) => {
                return Err(ApiError::Omi(OmiHookError::NoSuchUser(
                    ctx.req.omi_uid.clone(),
                )));
            }
        };

        if !actions_to_send.is_empty() {
            let tg_bot = TelegramBot::new(&ctx.env);
            let tg_user = TelegramAccount::load_by_user_id(&ctx.env, &user_id).await?;

            for action in actions_to_send {
                let message = match &tg_user.username {
                    Some(username) => format!(
                        "message from {} (@{}): {}",
                        tg_user.first_name, username, action.message
                    ),
                    None => format!("message from {}: {}", tg_user.first_name, action.message),
                };

                tracing::info!("Sending message to user {}: {}", ctx.req.omi_uid, message);

                tg_bot
                    .send_message(action.destination.kind.chat_id(), &message)
                    .await?;
            }
        } else {
            tracing::info!("No actions to send, payload: {:?}", ctx.req.payload);
        }

        Ok(())
    }
}

impl FromHttpRequest for OmiWebHookRequest {
    fn from_request(
        _env: worker::Env,
        req: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = ApiResult<Self>>>> {
        Box::pin(async move {
            let url = web_sys::Url::new(&req.uri().to_string()).unwrap();
            let search_params = url.search_params();
            match search_params.get("uid") {
                Some(mut omi_uid) => match json_body_to_any::<OmiPayload>(req.into_body()).await {
                    Ok(payload) => Ok(OmiWebHookRequest { omi_uid, payload }),
                    Err(e) => {
                        return Err(ApiError::Parse(e.to_string()));
                    }
                },
                None => Err(ApiError::Parse("uid not found".to_string())),
            }
        })
    }
}
