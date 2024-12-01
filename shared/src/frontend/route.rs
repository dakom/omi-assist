#[derive(Debug, Clone)]
pub enum Route {
    Landing(Landing),
    Dashboard(Dashboard),
    NotFound(NotFoundReason),
    TermsOfService,
    PrivacyPolicy,
}

impl Route {
    pub fn from_url(url: &str, root_path: &str) -> Self {
        let url = web_sys::Url::new(url).unwrap();
        let paths = url.pathname();
        let paths = paths
            .split('/')
            .into_iter()
            // skip all the roots (1 for the domain, 1 for each part of root path)
            .skip(root_path.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>();
        let paths = paths.as_slice();

        let uid = url.search_params().get("uid");

        match paths {
            [""] => Self::Landing(Landing::Welcome { uid }),
            ["no-auth"] => Self::NotFound(NotFoundReason::NoAuth),
            ["dashboard"] => Self::Dashboard(Dashboard::Actions),
            ["dashboard", dashboard_kind] => match *dashboard_kind {
                "actions" => Self::Dashboard(Dashboard::Actions),
                "destinations" => Self::Dashboard(Dashboard::Destinations),
                _ => Self::NotFound(NotFoundReason::BadUrl),
            },
            ["register", "start"] => Self::Landing(Landing::Auth(AuthRoute::RegisterStart { uid })),
            ["register", "complete", uid] => {
                Self::Landing(Landing::Auth(AuthRoute::RegisterComplete {
                    uid: uid.to_string(),
                }))
            }
            ["signin"] => Self::Landing(Landing::Auth(AuthRoute::Signin)),
            ["terms-of-service"] => Self::TermsOfService,
            ["privacy-policy"] => Self::PrivacyPolicy,
            // these usually aren't visited directly, but can be helpful for debugging
            _ => Self::NotFound(NotFoundReason::BadUrl),
        }
    }

    pub fn link_url(&self, domain: &str, root_path: &str) -> String {
        let s = if root_path.is_empty() {
            format!("{}/{}", domain, self.to_string())
        } else {
            format!("{}/{}/{}", domain, root_path, self.to_string())
        };

        s.trim_end_matches(r#"//"#).to_string()
    }

    // unlike backend auth, this is just a pure yes/no gate for frontend
    // so that it can redirect on unauthenticated pages
    pub fn requires_auth(&self) -> bool {
        match self {
            Self::Dashboard(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Route::Landing(landing) => match landing {
                Landing::Welcome { uid } => match uid {
                    Some(uid) => format!("/?uid={uid}"),
                    None => "/".to_string(),
                },
                Landing::Auth(auth_page) => match auth_page {
                    AuthRoute::RegisterStart { uid } => match uid {
                        Some(uid) => format!("register/start?uid={uid}"),
                        None => "register/start".to_string(),
                    },
                    AuthRoute::RegisterComplete { uid } => format!("register/complete/{uid}"),
                    AuthRoute::Signin => "signin".to_string(),
                },
            },
            Route::Dashboard(dashboard) => match dashboard {
                Dashboard::Actions => format!("dashboard/actions"),
                Dashboard::Destinations => format!("dashboard/destinations"),
            },
            Route::NotFound(reason) => match reason {
                NotFoundReason::BadUrl => "404".to_string(),
                NotFoundReason::NoAuth => "no-auth".to_string(),
            },
            Route::TermsOfService => "terms-of-service".to_string(),
            Route::PrivacyPolicy => "privacy-policy".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dashboard {
    Actions,
    Destinations,
}

#[derive(Debug, Clone)]
pub enum Landing {
    Welcome { uid: Option<String> },
    Auth(AuthRoute),
}

#[derive(Clone, Debug)]
pub enum AuthRoute {
    RegisterStart { uid: Option<String> },
    RegisterComplete { uid: String },
    Signin,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotFoundReason {
    NoAuth,
    BadUrl,
}
