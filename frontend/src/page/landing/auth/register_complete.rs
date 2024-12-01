use shared::{
    api::auth::{AuthRegister, AuthRegisterRequest, AuthRegisterResponse},
    auth::FRONTEND_ROUTE_AFTER_SIGNIN,
    backend::result::ApiError,
};

use crate::{page::landing::auth::TelegramUrlParams, prelude::*};

pub struct RegisterComplete {
    pub uid: String,
    pub error: Mutable<Option<ApiError>>,
}

impl RegisterComplete {
    pub fn new(uid: String) -> Arc<Self> {
        Arc::new(Self {
            uid,
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

                        let req = AuthRegisterRequest {
                            omi_uid: state.uid.clone(),
                            tg_uid: params.user_id,
                            data_check: params.data_check_string,
                            data_check_hash: params.data_check_hash
                        };

                        tracing::info!("registering with {:#?}", req);

                        match AuthRegister::fetch(req).await {
                            Ok(AuthRegisterResponse{uid, auth_key}) => {
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
