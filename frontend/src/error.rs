use std::sync::Arc;

use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::backend::result::{ApiError, AuthError};

use crate::{get_text, LOCALE};

pub trait ApiErrorExt {
    fn get_text(&self) -> String;
}

impl ApiErrorExt for ApiError {
    fn get_text(&self) -> String {
        // this goes through the fluent bindings
        let (id, args) = match self {
            Self::Auth(auth_error) => match auth_error {
                AuthError::OmiIdAlreadyExists => ("error-api-omi-id-already-exists", None),
                AuthError::TelegramIdAlreadyExists => {
                    ("error-api-telegram-id-already-exists", None)
                }
                AuthError::TelegramIdMismatch => ("error-api-telegram-id-mismatch", None),
                AuthError::NotAuthorized => ("error-api-not-authorized", None),
                AuthError::InvalidSignin => ("error-api-signin-invalid", None),
                AuthError::TermsNotAgreed => ("error-api-terms-not-agreed", None),
            },
            Self::MissingBody(_) => ("error-api-missing-body", None),
            Self::ParseBody(_) => ("error-api-parse-body", None),
            Self::Parse(_) => ("error-api-unknown-parse", None),
            Self::Unknown(_) => ("error-api-unknown", None),
            Self::Telegram(_) => ("error-api-telegram", None),
            Self::Omi(_) => ("error-api-omi", None),
            Self::Kv(_) => ("error-api-unknown", None),
            Self::Db(_) => ("error-api-unknown", None),
        };

        get_text!(id, args)
    }
}

// A component that makes it convenient to handle API errors for display
#[derive(Clone)]
pub struct ApiErrorDisplay {
    inner: Mutable<Option<Arc<ApiError>>>,
}

impl ApiErrorDisplay {
    pub fn new() -> Self {
        Self {
            inner: Mutable::new(None),
        }
    }

    pub fn set(&self, error: ApiError) {
        self.inner.set(Some(Arc::new(error)));
    }

    pub fn clear(&self) {
        self.inner.set(None);
    }

    pub fn text_signal(&self) -> impl Signal<Item = String> {
        self.inner
            .signal_cloned()
            .map(|err| err.as_ref().map(|err| err.get_text()).unwrap_or_default())
    }
}
