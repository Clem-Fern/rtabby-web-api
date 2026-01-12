mod env;
pub mod error;
pub mod models;
pub mod providers;
pub mod routes;
pub mod services;

use crate::env as app_env;

use actix_web::http::uri::Scheme;

use log::warn;
#[cfg(feature = "github-login")]
use providers::github;
#[cfg(feature = "gitlab-login")]
use providers::gitlab;
#[cfg(feature = "google-login")]
use providers::google;
#[cfg(feature = "microsoft-login")]
use providers::microsoft;
#[cfg(feature = "oidc-login")]
use providers::oidc;

use self::providers::OauthInfo;

#[derive(Clone, Debug)]
pub struct ProvidersConfig {
    pub https_callback: bool,
    pub available_providers: Vec<providers::Provider>,
}

impl ProvidersConfig {
    pub fn get_callback_scheme(&self) -> Scheme {
        if self.https_callback {
            Scheme::HTTPS
        } else {
            Scheme::HTTP
        }
    }
}

pub fn get_provider_config() -> ProvidersConfig {
    let https_callback = if app_env::var(env::ENV_HTTPS_CALLBACK).is_ok() {
        app_env::var(env::ENV_HTTPS_CALLBACK)
            .unwrap_or(String::from("false"))
            .to_lowercase()
            .parse()
            .unwrap_or(false)
    } else if app_env::var(env::ENV_USE_HTTPS).is_ok() {
        // DEPRECATED
        warn!("\"USE_HTTPS\" deprecated. Use \"HTTPS_CALLBACK\" instead.");
        app_env::var(env::ENV_USE_HTTPS).unwrap_or(String::from("0")) == "1"
    } else {
        false
    };

    let mut available_providers: Vec<providers::Provider> = vec![];

    #[cfg(feature = "github-login")]
    if app_env::var(github::env::ENV_GITHUB_APP_CLIENT_ID).is_ok()
        && app_env::var(github::env::ENV_GITHUB_APP_CLIENT_SECRET).is_ok()
    {
        available_providers.push(providers::Provider::Github(OauthInfo {
            client_id: app_env::var(github::env::ENV_GITHUB_APP_CLIENT_ID).unwrap(),
            client_secret: app_env::var(github::env::ENV_GITHUB_APP_CLIENT_SECRET).unwrap(),
        }));
    }

    #[cfg(feature = "gitlab-login")]
    if app_env::var(gitlab::env::ENV_GITLAB_APP_CLIENT_ID).is_ok()
        && app_env::var(gitlab::env::ENV_GITLAB_APP_CLIENT_SECRET).is_ok()
    {
        available_providers.push(providers::Provider::Gitlab(OauthInfo {
            client_id: app_env::var(gitlab::env::ENV_GITLAB_APP_CLIENT_ID).unwrap(),
            client_secret: app_env::var(gitlab::env::ENV_GITLAB_APP_CLIENT_SECRET).unwrap(),
        }));
    }

    #[cfg(feature = "google-login")]
    if app_env::var(google::env::ENV_GOOGLE_APP_CLIENT_ID).is_ok()
        && app_env::var(google::env::ENV_GOOGLE_APP_CLIENT_SECRET).is_ok()
    {
        available_providers.push(providers::Provider::Google(OauthInfo {
            client_id: app_env::var(google::env::ENV_GOOGLE_APP_CLIENT_ID).unwrap(),
            client_secret: app_env::var(google::env::ENV_GOOGLE_APP_CLIENT_SECRET).unwrap(),
        }));
    }

    #[cfg(feature = "microsoft-login")]
    if app_env::var(microsoft::env::ENV_MICROSOFT_APP_CLIENT_ID).is_ok()
        && app_env::var(microsoft::env::ENV_MICROSOFT_APP_CLIENT_SECRET).is_ok()
    {
        available_providers.push(providers::Provider::Microsoft(OauthInfo {
            client_id: app_env::var(microsoft::env::ENV_MICROSOFT_APP_CLIENT_ID).unwrap(),
            client_secret: app_env::var(microsoft::env::ENV_MICROSOFT_APP_CLIENT_SECRET).unwrap(),
        }));
    }

    #[cfg(feature = "oidc-login")]
    if app_env::var(oidc::env::ENV_OIDC_APP_CLIENT_ID).is_ok()
        && app_env::var(oidc::env::ENV_OIDC_APP_CLIENT_SECRET).is_ok()
    {
        available_providers.push(providers::Provider::Oidc(OauthInfo {
            client_id: app_env::var(oidc::env::ENV_OIDC_APP_CLIENT_ID).unwrap(),
            client_secret: app_env::var(oidc::env::ENV_OIDC_APP_CLIENT_SECRET).unwrap(),
        }));
    }

    ProvidersConfig {
        https_callback,
        available_providers,
    }
}
