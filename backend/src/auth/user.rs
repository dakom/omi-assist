//use super::durable_objects::token::{AuthTokenAfterValidation, AuthTokenDO, AuthTokenKind, AuthTokenValidateResponse};

use shared::{
    api::auth::{AuthTokenAfterValidation, AuthTokenKind, AuthTokenValidateResponse},
    auth::{HEADER_ADMIN_CODE, HEADER_ADMIN_UID, HEADER_AUTH_TOKEN_ID, HEADER_AUTH_TOKEN_KEY},
    backend::{
        result::{ApiError, ApiResult, AuthError},
        route::{Route, RouteAuthKind},
    },
    user::UserId,
};
use worker::{Env, HttpRequest};

use crate::{
    config::{AUTH_TOKEN_SIGNIN_EXPIRES, ENV_KEY_ADMIN_CODE},
    db::user::UserAccount,
    kv::auth::AuthKv,
};

pub struct AuthUser {
    pub account: UserAccount,
    pub token_id: String,
}

impl AuthUser {
    pub async fn try_new(
        env: &Env,
        req: &HttpRequest,
        route: &Route,
    ) -> ApiResult<Option<AuthUser>> {
        // early exit or get the auth token
        let user = match route.auth_kind() {
            RouteAuthKind::None | RouteAuthKind::NoAuthCookieSetter => None,
            auth_kind => match AuthUser::validate(&env, &req, auth_kind).await {
                Ok(user) => Some(user),
                Err(err) => {
                    tracing::info!("Auth error: {:?}", err);
                    // for clients we just want to say "not authorized"
                    // in case errors leak semi-sensitive info for debugging (like the nature of the auth keys, etc.)
                    return Err(ApiError::Auth(AuthError::NotAuthorized));
                }
            },
        };

        Ok(user)
    }

    async fn validate(
        env: &Env,
        req: &HttpRequest,
        auth_kind: RouteAuthKind,
    ) -> ApiResult<AuthUser> {
        // admin code is a special case, it's not a user token, it's a global admin code and should never be leaked
        let expected_admin_code = env
            .secret(ENV_KEY_ADMIN_CODE)
            .map(|s| s.to_string())
            .unwrap_or_default();
        let header_admin_code = req
            .headers()
            .get(HEADER_ADMIN_CODE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        let admin = if !expected_admin_code.is_empty() {
            expected_admin_code == header_admin_code
        } else {
            false
        };

        if admin {
            // allow admin to impersonate any user, and not need all the other cookie/header/token stuff
            let header_admin_uid = req
                .headers()
                .get(HEADER_ADMIN_UID)
                .and_then(|h| h.to_str().ok())
                .unwrap_or_default();
            if !header_admin_uid.is_empty() {
                let uid = UserId::new(
                    uuid::Uuid::parse_str(header_admin_uid)
                        .map_err(|_| ApiError::from("invalid admin uid"))?,
                );
                let account = match UserAccount::load(env, &uid).await {
                    Ok(account) => account,
                    Err(_) => {
                        tracing::warn!("admin uid not found: {:?} ... making user up!", uid);
                        UserAccount {
                            id: uid,
                            user_token: "".to_string(),
                            created_at: "".to_string(),
                        }
                    }
                };
                return Ok(AuthUser {
                    account,
                    token_id: "".to_string(),
                });
            }
        } else if matches!(auth_kind, RouteAuthKind::Admin) {
            return Err(ApiError::from("not an admin".to_string()));
        }

        // first try and get the token from the header, e.g. for non-browser clients who store the token_id securely
        let mut token_id = req
            .headers()
            .get(HEADER_AUTH_TOKEN_ID)
            .and_then(|s| s.to_str().ok())
            .unwrap_or_default()
            .to_string();

        if token_id.is_empty() {
            // Typical case is via browser, where we get token id from cookie (sent automatically via browser standards).
            // The cookie is httponly, so we can't read it from JS.
            //
            // This effectively prevents "XSS exfiltration" attacks, where a malicious script in the wild
            // can blindly dump local storage and send it to a different server for analysis.
            // That attack would work if the _only_ thing we checked was the token key (stored in local storage),
            // but we also check the token id, which is stored in a httponly cookie, so the attacker can't get it.
            let cookie_header = req
                .headers()
                .iter()
                .find_map(|(k, v)| {
                    if k.to_string().to_lowercase() == "cookie" {
                        Some(v.to_str().ok().unwrap_or_default())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();

            if cookie_header.contains(HEADER_AUTH_TOKEN_ID) {
                token_id = cookie_header
                    .split(";")
                    .map(|x| x.trim())
                    .find(|x| x.starts_with(HEADER_AUTH_TOKEN_ID))
                    .map(|x| x.split_once("=").map(|x| x.1.to_string()))
                    .flatten()
                    .unwrap_or_default();
            }
        }

        if token_id.is_empty() {
            return Err(ApiError::from("missing token id".to_string()));
        }

        // token key is always from header
        // this effectively prevents "CSRF" attacks, where a malicious site can trick a user into making a request
        // to our server, but the unlike the token id, the token key is not stored in a cookie
        // and so it is not sent automatically by the browser
        //
        // in other words - to be successful, the attacker would need to *both*:
        // 1. trick the user into sending a request via browser (which will send the token_id cookie)
        // 2. trick the user into sending the token key (which requires reading local storage)
        //
        // that level of access is indistinguishable from the real user
        let token_key = req
            .headers()
            .get(HEADER_AUTH_TOKEN_KEY)
            .and_then(|s| s.to_str().ok())
            .unwrap_or_default()
            .to_string();

        if token_key.is_empty() {
            return Err(ApiError::from("missing token key".to_string()));
        }

        // validate the token id and key
        let AuthTokenValidateResponse { uid, user_token } = AuthKv::validate(
            env,
            AuthTokenKind::Signin,
            &token_id,
            token_key.to_string(),
            AuthTokenAfterValidation::ExtendExpiresMs(AUTH_TOKEN_SIGNIN_EXPIRES),
        )
        .await?;

        let account = UserAccount::load(env, &uid).await?;

        match auth_kind {
            // no need to handle all the variants here, we've early-exited for non-auth routes
            // and anyway we end up with a strict fallback of at least getting a valid token and user id
            RouteAuthKind::PartialAuthTokenOnly => {
                // no further validation needed, having a valid token is enough to destroy it
            }
            _ => {
                // validate the user token - if mismatched, user has "logged out everywhere"
                if account.user_token != user_token {
                    return Err(format!("user token mismatch for user id {uid}").into());
                }
            }
        }

        Ok(AuthUser { account, token_id })
    }
}
