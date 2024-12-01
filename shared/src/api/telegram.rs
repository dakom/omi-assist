use http::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::backend::route::Route;

use super::ApiReq;

#[derive(Debug)]
pub enum TelegramBotCommand {
    Start,
    Omi(TelegramOmiCommand),
}

#[derive(Debug)]
pub enum TelegramOmiCommand {
    LinkDm,
    LinkGroup,
}

#[derive(Serialize, Deserialize, Error, Debug, Clone, PartialEq)]
pub enum TelegramBotError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal telegram error: {0}")]
    Internal(String),

    #[error("Unknown non-Omi command")]
    UnknownCommand(String),

    #[error("Bad Omi command {0}")]
    OmiCommand(String),

    #[error("Unsupported message")]
    UnsupportedMessage,
}

impl TryFrom<&TelegramMessage> for TelegramBotCommand {
    type Error = TelegramBotError;

    fn try_from(message: &TelegramMessage) -> Result<Self, Self::Error> {
        if let Some(text) = message.text.clone() {
            let parts = text.split_whitespace().collect::<Vec<&str>>();

            if parts.len() > 0 {
                match message.chat.chat_type {
                    TelegramChatType::Private => match parts[0] {
                        "/start" => return Ok(TelegramBotCommand::Start),
                        "/link" => return Ok(TelegramBotCommand::Omi(TelegramOmiCommand::LinkDm)),
                        _ => return Err(TelegramBotError::OmiCommand(text)),
                    },
                    _ => {
                        if parts[0] == "/omi" && parts.len() > 1 {
                            match &parts[1..] {
                                ["link"] => {
                                    return Ok(TelegramBotCommand::Omi(
                                        TelegramOmiCommand::LinkGroup,
                                    ))
                                }
                                _ => return Err(TelegramBotError::OmiCommand(text)),
                            }
                        }
                    }
                }
            }
            return Err(TelegramBotError::UnknownCommand(text));
        }
        Err(TelegramBotError::UnsupportedMessage)
    }
}

pub struct TelegramWebHook {}

impl ApiReq for TelegramWebHook {
    const ROUTE: Route = Route::TelegramWebHook;
    const METHOD: Method = Method::POST;

    type Req = TelegramWebHookRequest;
}

// https://core.telegram.org/bots/api#update
#[derive(Deserialize, Serialize, Debug)]
pub struct TelegramWebHookRequest {
    pub update_id: i64,
    pub message: Option<TelegramMessage>,
    pub edited_message: Option<TelegramMessage>,
    pub channel_post: Option<TelegramMessage>,
    pub edited_channel_post: Option<TelegramMessage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub from: TelegramUser,
    pub chat: TelegramChat,
    pub date: u64,
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TelegramUser {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TelegramChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: TelegramChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum TelegramChatType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    SuperGroup,
    #[serde(rename = "channel")]
    Channel,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TelegramWebHookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: u64,
    pub ip_address: Option<String>,
    pub last_error_date: Option<u64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<u64>,
    pub max_connections: Option<u64>,
    pub allowed_updates: Option<Vec<String>>,
}
