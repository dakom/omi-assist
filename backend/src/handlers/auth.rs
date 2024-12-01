use crate::{
    any_to_json_response,
    api_ext::*,
    config::{AUTH_TOKEN_SIGNIN_EXPIRES, ENV_KEY_TELEGRAM_AUTH_TOKEN},
    db::user::{OmiAccount, TelegramAccount, UserAccount},
    empty_response,
    kv::auth::AuthKv,
    ApiContext,
};
use async_trait::async_trait;
use auth::{
    AuthCheck, AuthCheckResponse, AuthRegister, AuthRegisterRequest, AuthRegisterResponse,
    AuthSignin, AuthSigninRequest, AuthSigninResponse, AuthSignout, AuthSignoutRequest,
    AuthTokenCreateResponse, AuthTokenKind,
};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use shared::{
    api::*,
    auth::HEADER_AUTH_TOKEN_ID,
    backend::result::{ApiError, ApiResult, AuthError},
    user::UserId,
};
use worker::{Env, HttpRequest, HttpResponse};

// Register
#[async_trait(?Send)]
impl ApiBothWithExtraExt for AuthRegister {
    type Req = <Self as ApiBoth>::Req;
    type Res = <Self as ApiBoth>::Res;
    type Extra = AuthTokenCreateResponse;

    async fn handle(
        ctx: &ApiContext<AuthRegisterRequest>,
    ) -> ApiResult<(AuthRegisterResponse, AuthTokenCreateResponse)> {
        let AuthRegisterRequest {
            omi_uid,
            tg_uid,
            data_check,
            data_check_hash,
        } = &ctx.req;

        if !data_check.contains(&format!("id={tg_uid}")) {
            return Err(AuthError::TelegramIdMismatch.into());
        }

        validate_telegram_login(&ctx.env, data_check, data_check_hash)?;

        if OmiAccount::exists(&ctx.env, &omi_uid).await? {
            return Err(AuthError::OmiIdAlreadyExists.into());
        }

        if TelegramAccount::exists(&ctx.env, *tg_uid).await? {
            return Err(AuthError::TelegramIdAlreadyExists.into());
        }

        register(&ctx.env, omi_uid, *tg_uid).await
    }

    async fn response(
        _ctx: &ApiContext<AuthRegisterRequest>,
        data: AuthRegisterResponse,
        auth_token: AuthTokenCreateResponse,
    ) -> HttpResponse {
        let mut res = any_to_json_response(&data, None).await;
        set_signin_cookie(&mut res, &auth_token.id);
        res
    }
}

impl FromHttpRequest for AuthRegisterRequest {}

pub async fn register(
    env: &Env,
    omi_uid: &str,
    tg_uid: i64,
) -> ApiResult<(AuthRegisterResponse, AuthTokenCreateResponse)> {
    // Register in database
    let uid = UserId::new(uuid::Uuid::now_v7());
    let user_token = uuid::Uuid::now_v7().as_simple().to_string();
    UserAccount::insert(&env, &uid, &user_token).await?;
    OmiAccount::insert(&env, &omi_uid, &uid).await?;
    TelegramAccount::insert(&env, tg_uid, &uid).await?;

    // Log user in
    let auth_token = AuthKv::create(
        &env,
        AuthTokenKind::Signin,
        uid.clone(),
        user_token.clone(),
        AUTH_TOKEN_SIGNIN_EXPIRES,
    )
    .await?;
    let auth_key = auth_token.key.clone();

    Ok((AuthRegisterResponse { uid, auth_key }, auth_token))
}

// Signin
#[async_trait(?Send)]
impl ApiBothWithExtraExt for AuthSignin {
    type Req = <Self as ApiBoth>::Req;
    type Res = <Self as ApiBoth>::Res;
    type Extra = AuthTokenCreateResponse;

    async fn handle(
        ctx: &ApiContext<AuthSigninRequest>,
    ) -> ApiResult<(AuthSigninResponse, AuthTokenCreateResponse)> {
        let AuthSigninRequest {
            tg_uid,
            data_check,
            data_check_hash,
        } = &ctx.req;

        validate_telegram_login(&ctx.env, data_check, data_check_hash)?;

        let tg_account = TelegramAccount::load(&ctx.env, *tg_uid).await?;
        let user = UserAccount::load(&ctx.env, &tg_account.user_id).await?;

        // Log user in
        let auth_token = AuthKv::create(
            &ctx.env,
            AuthTokenKind::Signin,
            user.id.clone(),
            user.user_token.clone(),
            AUTH_TOKEN_SIGNIN_EXPIRES,
        )
        .await?;
        let auth_key = auth_token.key.clone();

        Ok((
            AuthSigninResponse {
                uid: user.id,
                auth_key,
            },
            auth_token,
        ))
    }

    async fn response(
        _ctx: &ApiContext<AuthSigninRequest>,
        data: AuthSigninResponse,
        auth_token: AuthTokenCreateResponse,
    ) -> HttpResponse {
        let mut res = any_to_json_response(&data, None).await;
        set_signin_cookie(&mut res, &auth_token.id);
        res
    }
}

impl FromHttpRequest for AuthSigninRequest {}

// Signout
#[async_trait(?Send)]
impl ApiReqExt for AuthSignout {
    type Req = <Self as ApiReq>::Req;

    async fn handle(ctx: &ApiContext<AuthSignoutRequest>) -> ApiResult<()> {
        let AuthSignoutRequest { everywhere } = &ctx.req;

        // safe, signout requires that the auth_token in kv was validated
        let user = ctx.user.as_ref().unwrap();

        AuthKv::delete(&ctx.env, AuthTokenKind::Signin, &user.token_id).await?;

        if *everywhere {
            let user_token = uuid::Uuid::now_v7().as_simple().to_string();
            UserAccount::update_user_token(&ctx.env, &user.account.id, &user_token).await?;
        }

        Ok(())
    }

    async fn response(_ctx: &ApiContext<AuthSignoutRequest>) -> HttpResponse {
        let mut res = empty_response(None);
        delete_signin_cookie(&mut res);
        res
    }
}

impl FromHttpRequest for AuthSignoutRequest {}

// Check
#[async_trait(?Send)]
impl ApiResExt for AuthCheck {
    type Res = <Self as ApiRes>::Res;

    async fn handle(ctx: &ApiContext<HttpRequest>) -> ApiResult<AuthCheckResponse> {
        let uid = ctx.uid_unchecked();
        Ok(AuthCheckResponse { uid })
    }
}

fn validate_telegram_login(env: &Env, data_check: &str, data_check_hash: &str) -> ApiResult<()> {
    let auth_token = env.secret(ENV_KEY_TELEGRAM_AUTH_TOKEN).unwrap().to_string();

    // Validate the tg hash (https://core.telegram.org/widgets/login#checking-authorization)
    let mut sha256 = Sha256::new();

    sha256.update(auth_token);
    let secret_key = sha256.finalize();

    let mut mac =
        Hmac::<Sha256>::new_from_slice(&secret_key).expect("HMAC can take key of any size");
    mac.update(data_check.as_bytes());
    let result = mac.finalize();
    let computed_hash = hex::encode(result.into_bytes());

    if computed_hash != *data_check_hash {
        tracing::warn!("hash is invalid {computed_hash} vs. {data_check_hash}");
        Err(ApiError::Auth(AuthError::NotAuthorized))
    } else {
        Ok(())
    }
}

#[cfg(debug_assertions)]
pub fn set_signin_cookie(res: &mut HttpResponse, auth_token_id: &str) {
    let value = format!("{HEADER_AUTH_TOKEN_ID}={auth_token_id}; Path=/; HttpOnly; Secure; Partitioned; SameSite=None; Max-Age=2147483647");
    res.headers_mut()
        .insert("Set-Cookie", value.parse().unwrap());
}

// since our api server is on a different domain
// we need samesite=none
// see https://www.troyhunt.com/promiscuous-cookies-and-their-impending-death-via-the-samesite-policy/
#[cfg(not(debug_assertions))]
pub fn set_signin_cookie(res: &mut HttpResponse, auth_token_id: &str) {
    let value = format!("{HEADER_AUTH_TOKEN_ID}={auth_token_id}; Path=/; HttpOnly; Secure; Partitioned; SameSite=None; Max-Age=2147483647");
    res.headers_mut()
        .insert("Set-Cookie", value.parse().unwrap());
}

pub fn delete_signin_cookie(res: &mut HttpResponse) {
    let value = format!("{HEADER_AUTH_TOKEN_ID}=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT");
    res.headers_mut()
        .insert("Set-Cookie", value.parse().unwrap());
}
