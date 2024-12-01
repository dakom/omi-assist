use crate::{page::landing::auth::TelegramUrlParams, prelude::*};
use shared::{
    api::auth::{AuthSignin, AuthSigninRequest, AuthSigninResponse},
    auth::FRONTEND_ROUTE_AFTER_SIGNIN,
    backend::result::ApiError,
};

pub struct Signin {
    pub error: Mutable<Option<ApiError>>,
}

impl Signin {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            error: Mutable::new(None),
        })
    }
    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("gap", "1.3rem")
                .style("align-items", "center")
                .style("margin-top", "5.25rem")
                .style("text-align", "center")
            }
        });

        html!("div", {
            .class(&*CLASS)
            .future(clone!(state => async move {
                match TelegramUrlParams::new() {
                    Ok(params) => {
                        let req = AuthSigninRequest {
                            tg_uid: params.user_id,
                            data_check: params.data_check_string,
                            data_check_hash: params.data_check_hash
                        };

                        match AuthSignin::fetch(req).await {
                            Ok(AuthSigninResponse{uid, auth_key}) => {
                                match AUTH.on_signin(uid, auth_key).await {
                                    Ok(_) => {
                                        FRONTEND_ROUTE_AFTER_SIGNIN.go_to_url();
                                    },
                                    Err(err) => {
                                        state.error.set(Some(err));
                                    }
                                }
                            },
                            Err(err) => {
                                state.error.set(Some(err));
                            }
                        }
                    },
                    Err(err) => {
                        state.error.set(Some(ApiError::Unknown(err.to_string())));
                    }
                }
            }))
            .child_signal(state.error.signal_cloned().map(|error| {
                match error {
                    Some(err) => {
                        Some(html!("div", {
                            .class([FontSize::H2.class(), ColorText::Error.class()])
                            .text(&err.to_string())
                        }))
                    },
                    None => {
                        Some(html!("div", {
                            .class([FontSize::H1.class(), ColorText::Header.class()])
                            .text(&get_text!("landing-loading"))
                        }))
                    }
                }
            }))
        })
    }
}
