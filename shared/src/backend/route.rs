#[derive(Debug, Clone)]
pub enum Route {
    Info,
    Auth(AuthRoute),
    Admin(AdminRoute),
    Action(ActionRoute),
    TelegramWebHook,
    OmiWebHook,
}

#[derive(Debug, Clone)]
pub enum AuthRoute {
    Register,
    Signin,
    Check,
    Signout,
}

#[derive(Debug, Clone)]
pub enum AdminRoute {
    TelegramSetWebHook,
    PopulateFakeUser,
}

#[derive(Debug, Clone)]
pub enum ActionRoute {
    ListDestinations,
    AddAction,
    DeleteAction,
    ListActions,
}

impl Route {
    pub fn try_from_url(url: &str, root_path: &str) -> Option<Self> {
        let url = web_sys::Url::new(url).unwrap();
        let paths = url.pathname();
        let paths = paths
            .split('/')
            .into_iter()
            // skip all the roots (1 for the domain, 1 for each part of root path)
            .skip(root_path.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>();
        let paths = paths.as_slice();

        match paths {
            ["auth", auth_path @ ..] => AuthRoute::try_from_paths(auth_path).map(Self::Auth),
            ["admin", admin_path @ ..] => AdminRoute::try_from_paths(admin_path).map(Self::Admin),
            ["action", action_path @ ..] => {
                ActionRoute::try_from_paths(action_path).map(Self::Action)
            }
            ["info"] => Some(Self::Info),
            ["tg"] => Some(Self::TelegramWebHook),
            ["omi"] => Some(Self::OmiWebHook),
            _ => None,
        }
    }

    // in http://example.com/foo/bar/baz
    // domain = http://example.com
    // root_path = foo
    // the route itself would map to bar/baz
    pub fn link(&self, domain: &str, root_path: &str) -> String {
        if root_path.is_empty() {
            format!("{}/{}", domain, self.to_string())
        } else {
            format!("{}/{}/{}", domain, root_path, self.to_string())
        }
    }

    pub fn auth_kind(&self) -> RouteAuthKind {
        match self {
            Route::Auth(auth_route) => match auth_route {
                AuthRoute::Check => RouteAuthKind::Full,
                // these just need to set the cookie, no auth checks
                AuthRoute::Register => RouteAuthKind::NoAuthCookieSetter,
                AuthRoute::Signin => RouteAuthKind::NoAuthCookieSetter,
                // signout is allowed even if we've already "signed out everywhere"
                AuthRoute::Signout => RouteAuthKind::PartialAuthTokenOnly,
            },
            Route::Action(_) => RouteAuthKind::Full,
            Route::Admin(_) => RouteAuthKind::Admin,
            Route::Info => RouteAuthKind::None,
            Route::TelegramWebHook => RouteAuthKind::None,
            Route::OmiWebHook => RouteAuthKind::None,
        }
    }
}

impl AuthRoute {
    pub fn try_from_paths(paths: &[&str]) -> Option<Self> {
        match *paths {
            ["register"] => Some(Self::Register),
            ["check"] => Some(Self::Check),
            ["signout"] => Some(Self::Signout),
            ["signin"] => Some(Self::Signin),
            _ => None,
        }
    }
}

impl AdminRoute {
    pub fn try_from_paths(paths: &[&str]) -> Option<Self> {
        match *paths {
            ["tg", "set-web-hook"] => Some(Self::TelegramSetWebHook),
            ["populate-fake-user"] => Some(Self::PopulateFakeUser),
            _ => None,
        }
    }
}

impl ActionRoute {
    pub fn try_from_paths(paths: &[&str]) -> Option<Self> {
        match *paths {
            ["list-destinations"] => Some(Self::ListDestinations),
            ["add-action"] => Some(Self::AddAction),
            ["delete-action"] => Some(Self::DeleteAction),
            ["list-actions"] => Some(Self::ListActions),
            _ => None,
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::Auth(auth_route) => {
                format!("auth/{}", auth_route)
            }
            Self::Admin(admin_route) => {
                format!("admin/{}", admin_route)
            }
            Self::Action(action_route) => {
                format!("action/{}", action_route)
            }
            Self::Info => "info".to_string(),
            Self::TelegramWebHook => "tg".to_string(),
            Self::OmiWebHook => "omi".to_string(),
        };

        write!(f, "{}", s)
    }
}
impl std::fmt::Display for AuthRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::Register => "register".to_string(),
            Self::Signout => "signout".to_string(),
            Self::Check => "check".to_string(),
            Self::Signin => "signin".to_string(),
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for AdminRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::TelegramSetWebHook => "tg/set-web-hook".to_string(),
            Self::PopulateFakeUser => "populate-fake-user".to_string(),
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for ActionRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::ListDestinations => "list-destinations".to_string(),
            Self::AddAction => "add-action".to_string(),
            Self::DeleteAction => "delete-action".to_string(),
            Self::ListActions => "list-actions".to_string(),
        };

        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Debug)]
pub enum RouteAuthKind {
    /// No credentials sent or needed at all, plain ol' public access
    None,
    /// Admin-only
    Admin,
    /// Full protection
    /// token, user_token, and email must all be validated
    Full,
    /// Just the ability to send and set cookies, tokens aren't checked at all
    /// e.g. for the signin/register routes, which must allow backend to set the cookie
    NoAuthCookieSetter,
    /// All credentials are sent, but only auth token is validated
    /// user_token is not checked
    /// e.g. called from signout route so that the auth token can be invalidated on that device
    /// regardless of whether the session is still active across other devices
    /// but we don't want to allow signout for arbitrary users
    PartialAuthTokenOnly,
    // All credentials are sent, all tokens are verified, but current email is not verified
    // e.g. for validate email flow itself (not used if email isn't a thing system-wide)
    //PartialAuthAndUserTokenOnly,
}
