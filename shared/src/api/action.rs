use std::fmt::Display;

use crate::backend::route::{ActionRoute, Route};
use http::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use super::{ApiBoth, ApiReq};

// List Action Destinations
pub struct ListActionDestinations {}

impl ApiBoth for ListActionDestinations {
    const ROUTE: Route = Route::Action(ActionRoute::ListDestinations);
    const METHOD: Method = Method::POST;

    type Req = ListActionDestinationsRequest;
    type Res = ListActionDestinationsResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListActionDestinationsRequest {
    // TODO
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListActionDestinationsResponse {
    pub destinations: Vec<ActionDestination>,
}

// Add Action
pub struct AddAction {}

impl ApiBoth for AddAction {
    const ROUTE: Route = Route::Action(ActionRoute::AddAction);
    const METHOD: Method = Method::POST;

    type Req = AddActionRequest;
    type Res = AddActionResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddActionRequest {
    pub destination_id: ActionDestinationId,
    pub prompt: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddActionResponse {
    pub action: Action,
}

// Delete Action
pub struct DeleteAction {}

impl ApiReq for DeleteAction {
    const ROUTE: Route = Route::Action(ActionRoute::DeleteAction);
    const METHOD: Method = Method::POST;

    type Req = DeleteActionRequest;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteActionRequest {
    pub id: ActionId,
}

// List Actions

pub struct ListActions {}

impl ApiBoth for ListActions {
    const ROUTE: Route = Route::Action(ActionRoute::ListActions);
    const METHOD: Method = Method::POST;

    type Req = ListActionsRequest;
    type Res = ListActionsResponse;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListActionsRequest {
    // TODO
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListActionsResponse {
    pub actions: Vec<Action>,
}

// Data types

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub id: ActionId,
    pub destination: ActionDestination,
    pub prompt: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionDestination {
    pub id: ActionDestinationId,
    pub name: String,
    pub kind: ActionDestinationKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionDestinationKind {
    TelegramDm { chat_id: i64 },
    TelegramGroup { chat_id: i64 },
}

impl ActionDestinationKind {
    pub fn chat_id(&self) -> i64 {
        match self {
            ActionDestinationKind::TelegramDm { chat_id } => *chat_id,
            ActionDestinationKind::TelegramGroup { chat_id } => *chat_id,
        }
    }
}

// TODO - make a macro for UUID newtype wrappers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ActionId(Uuid);

impl ActionId {
    pub fn new(u: Uuid) -> Self {
        Self(u)
    }

    pub fn to_string(&self) -> String {
        self.0.simple().to_string()
    }
}

impl From<&ActionId> for JsValue {
    fn from(u: &ActionId) -> Self {
        u.to_string().into()
    }
}

impl From<ActionId> for JsValue {
    fn from(u: ActionId) -> Self {
        u.to_string().into()
    }
}

impl TryFrom<&str> for ActionId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(value) {
            Ok(u) => Ok(Self(u)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl TryFrom<String> for ActionId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Display for ActionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

// TODO - make a macro for UUID newtype wrappers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ActionDestinationId(Uuid);

impl ActionDestinationId {
    pub fn new(u: Uuid) -> Self {
        Self(u)
    }

    pub fn to_string(&self) -> String {
        self.0.simple().to_string()
    }
}

impl From<&ActionDestinationId> for JsValue {
    fn from(u: &ActionDestinationId) -> Self {
        u.to_string().into()
    }
}

impl From<ActionDestinationId> for JsValue {
    fn from(u: ActionDestinationId) -> Self {
        u.to_string().into()
    }
}

impl TryFrom<&str> for ActionDestinationId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(value) {
            Ok(u) => Ok(Self(u)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl TryFrom<String> for ActionDestinationId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Display for ActionDestinationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
