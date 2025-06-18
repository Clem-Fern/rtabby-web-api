use crate::login::error::OauthError;
use crate::login::providers::{get_access_token, get_user_info, OauthInfo, OauthUserInfo};
use actix_web::http::uri::Scheme;

pub mod env {
    pub const ENV_GITLAB_APP_CLIENT_ID: &str = "GITLAB_APP_CLIENT_ID";
    pub const ENV_GITLAB_APP_CLIENT_SECRET: &str = "GITLAB_APP_CLIENT_SECRET";
}

pub const GITLAB_OAUTH_AUTHORIZE_URL: &str = "https://gitlab.com/oauth/authorize";
pub const GITLAB_OAUTH_ACCESS_TOKEN_URL: &str = "https://gitlab.com/oauth/token";
pub const GITLAB_OAUTH_USER_INFO_URL: &str = "https://gitlab.com/api/v4/user";

pub type GitlabOauthUserInfo = OauthUserInfo<i32, String>;

pub async fn user_info(
    scheme: Scheme,
    oauth: &OauthInfo,
    host: String,
    token: String,
) -> Result<GitlabOauthUserInfo, OauthError> {
    let redirect_uri = format!("{}://{}/login/gitlab/callback", scheme, host);
    let token = get_access_token(
        GITLAB_OAUTH_ACCESS_TOKEN_URL,
        token,
        oauth.client_id.clone(),
        oauth.client_secret.clone(),
        "authorization_code",
        Some(redirect_uri),
    )
    .await?;
    get_user_info(GITLAB_OAUTH_USER_INFO_URL, token)
        .await
        .map_err(OauthError::UserInfo)?
        .json::<GitlabOauthUserInfo>()
        .await
        .map_err(OauthError::UserInfo)
}
