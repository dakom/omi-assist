use awsm_web::env::env_var;

use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    // the part of the url that is not the domain
    // e.g. in http://example.com/foo/bar, this would be "foo" if we want
    // all parsing to start from /bar
    // it's helpful in shared hosting environments where the app is not at the root
    pub frontend_domain: &'static str,
    pub frontend_root_path: &'static str,
    pub media_root: &'static str,
    pub default_lang: Option<&'static str>,
    pub api_domain: &'static str,
    pub api_root_path: &'static str,
    // see usage and comments in auth, this is fine
    pub auth_signin_key_storage_name: &'static str,
    pub admin_login: Option<AdminLogin>,
}

#[derive(Debug)]
pub struct AdminLogin {
    pub admin_code: String,
    pub admin_uid: String,
}

impl AdminLogin {
    #[cfg(all(feature = "dev", debug_assertions))]
    pub fn debug_dev() -> Option<Self> {
        let admin_code = load_backend_secrets()
            .unwrap()
            .lines()
            .find(|line| line.starts_with("ADMIN_CODE="))
            .map(|line| line.split('=').last().unwrap().to_string())
            .unwrap();
        let admin_code = admin_code.replace("\"", "");

        Some(Self {
            admin_code,
            admin_uid: "019377c1-dc7a-7591-8bc6-b16c913fec34".to_string(),
        })
    }

    #[cfg(not(all(feature = "dev", debug_assertions)))]
    pub fn debug_dev() -> Option<Self> {
        None
    }
}

fn load_backend_secrets() -> Option<&'static str> {
    #[cfg(all(feature = "dev", debug_assertions))]
    {
        Some(include_str!("../../backend/.dev.vars"))
    }
    #[cfg(not(all(feature = "dev", debug_assertions)))]
    {
        None
    }
}

impl Config {
    pub fn app_image_url(&self, path: &str) -> String {
        format!("{}/{}", self.media_root, path)
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub const CONFIG: LazyLock<Config> = LazyLock::new(|| {
            Config {
                frontend_domain: "http://localhost:8080",
                frontend_root_path: "",
                media_root: "http://localhost:9000",
                //default_lang: Some("he-IL")
                default_lang: None,
                api_domain: "http://localhost:8787",
                api_root_path: "",
                auth_signin_key_storage_name: "omi_assist_auth_signin_key",
                admin_login: AdminLogin::debug_dev()
            }
        });
    } else {
        pub const CONFIG: LazyLock<Config> = LazyLock::new(|| {
            Config {
                frontend_domain: "https://omi-assist.pages.dev",
                frontend_root_path: "",
                media_root: "/media",
                default_lang: None,
                api_domain: "https://omi-assist-api-prod.dakom.workers.dev",
                api_root_path: "",
                auth_signin_key_storage_name: "omi_assist_auth_signin_key",
                admin_login: None,
            }
        });
    }
}

#[allow(dead_code)]
fn get_env(name: &str) -> Option<String> {
    match env_var(name) {
        Ok(value) => {
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        Err(_) => None,
    }
}
