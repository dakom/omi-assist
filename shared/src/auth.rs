use crate::frontend::route::{Dashboard, Route as FrontendRoute};

pub const HEADER_AUTH_TOKEN_ID: &str = "X-OMI-TOKEN-ID";
pub const HEADER_AUTH_TOKEN_KEY: &str = "X-OMI-TOKEN-KEY";
pub const HEADER_ADMIN_CODE: &'static str = "X-OMI-ADMIN-CODE";
pub const HEADER_ADMIN_UID: &'static str = "X-OMI-ADMIN-UID"; // allows admin to impersonate another user

pub const FRONTEND_ROUTE_AFTER_SIGNIN: FrontendRoute = FrontendRoute::Dashboard(Dashboard::Actions);
