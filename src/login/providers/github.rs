use crate::login::error::OauthError;
use crate::login::providers::{get_user_info, get_access_token, OauthInfo, OauthUserInfo};

pub mod env {
    pub const ENV_GITHUB_APP_CLIENT_ID: &str = "GITHUB_APP_CLIENT_ID";
    pub const ENV_GITHUB_APP_CLIENT_SECRET: &str = "GITHUB_APP_CLIENT_SECRET";
}

pub const GITHUB_OAUTH_AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
pub const GITHUB_OAUTH_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
pub const GITHUB_OAUTH_USER_INFO_URL: &str = "https://api.github.com/user";

pub type GithubOauthUserInfo = OauthUserInfo<i32, String>;

pub async fn user_info(oauth: &OauthInfo, token: String) -> Result<GithubOauthUserInfo, OauthError> {
    let token = get_access_token(GITHUB_OAUTH_ACCESS_TOKEN_URL, token, oauth.client_id.clone(), oauth.client_secret.clone(), "authorization_code", None).await?;
    get_user_info(GITHUB_OAUTH_USER_INFO_URL, token).await.map_err(OauthError::UserInfo)?.json::<GithubOauthUserInfo>().await.map_err(OauthError::UserInfo)
}