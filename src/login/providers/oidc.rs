use std::sync::LazyLock;

use crate::env as app_env;
use crate::login::error::OauthError;
use crate::login::providers::{get_access_token, get_user_info, OauthInfo, OauthUserInfo};
use actix_web::http::uri::Scheme;
use serde::Deserialize;
use tokio::sync::OnceCell;

pub mod env {
    pub const ENV_OIDC_APP_CLIENT_ID: &str = "OIDC_APP_CLIENT_ID";
    pub const ENV_OIDC_APP_CLIENT_SECRET: &str = "OIDC_APP_CLIENT_SECRET";
    pub const ENV_OIDC_APP_CONFIG_URL: &str = "OIDC_APP_CONFIG_URL";
}

static OIDC_CONFIG_CELL: LazyLock<OnceCell<OidcConfiguration>> = LazyLock::new(|| OnceCell::new());


#[derive(Deserialize)]
pub struct OidcConfiguration {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
}

pub async fn get_oidc_config() -> Result<&'static OidcConfiguration, OauthError> {
    OIDC_CONFIG_CELL.get_or_try_init(|| async {
        fetch_oidc_config().await
    }).await
}

pub async fn fetch_oidc_config() -> Result<OidcConfiguration, OauthError> {
    let client = reqwest::Client::new();
    let well_known_url = app_env::var(env::ENV_OIDC_APP_CONFIG_URL).unwrap();
    let res = client
        .get(&well_known_url)
        .send()
        .await
        .map_err(OauthError::OIDCConfiguration)?;
    let oidc_config = res
        .json::<OidcConfiguration>()
        .await
        .map_err(OauthError::OIDCConfiguration)?;

    Ok(oidc_config)
}

pub type OidcUserInfo = OauthUserInfo<String, String>;

pub async fn user_info(
    scheme: Scheme,
    oauth: &OauthInfo,
    host: String,
    token: String,
) -> Result<OidcUserInfo, OauthError> {
    let oidc_config = get_oidc_config().await?;
    let redirect_uri = format!("{}://{}/login/oidc/callback", scheme, host);

    let token = get_access_token(
        &oidc_config.token_endpoint,
        token,
        oauth.client_id.clone(),
        oauth.client_secret.clone(),
        "authorization_code",
        Some(redirect_uri),
    )
    .await?;
    get_user_info(&oidc_config.userinfo_endpoint, token)
        .await
        .map_err(OauthError::UserInfo)?
        .json::<OidcUserInfo>()
        .await
        .map_err(OauthError::UserInfo)
}
