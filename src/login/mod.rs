mod env;
pub mod models;
pub mod providers;
pub mod routes;
pub mod services;
pub mod error;
mod tools;

use crate::env as app_env;

#[cfg(feature = "github-login")]
use providers::github;
#[cfg(feature = "gitlab-login")]
use providers::gitlab;
#[cfg(feature = "google-login")]
use providers::google;
#[cfg(feature = "microsoft-login")]
use providers::microsoft;

use self::providers::OauthInfo;

#[derive(Clone, Debug)]
pub struct ProvidersConfig {
    pub available_providers: Vec<providers::Provider>,
}

pub fn get_provider_config() -> ProvidersConfig {
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

    ProvidersConfig {
        available_providers
    }
}
