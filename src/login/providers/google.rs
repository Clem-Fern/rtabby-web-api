use crate::login::error::OauthError;
use crate::login::providers::{get_access_token, get_user_info, OauthInfo, OauthUserInfo};
use actix_web::http::uri::Scheme;

pub mod env {
    pub const ENV_GOOGLE_APP_CLIENT_ID: &str = "GOOGLE_APP_CLIENT_ID";
    pub const ENV_GOOGLE_APP_CLIENT_SECRET: &str = "GOOGLE_APP_CLIENT_SECRET";
}

pub const GOOGLE_OAUTH_AUTHORIZE_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_OAUTH_ACCESS_TOKEN_URL: &str = "https://accounts.google.com/o/oauth2/token";
pub const GOOGLE_OAUTH_USER_INFO_URL: &str = "https://www.googleapis.com/oauth2/v1/userinfo";

pub type GoogleOauthUserInfo = OauthUserInfo;

pub async fn user_info(
    scheme: Scheme,
    oauth: &OauthInfo,
    host: String,
    code: String,
) -> Result<GoogleOauthUserInfo, OauthError> {
    let redirect_uri = format!("{}://{}/login/google/callback", scheme, host);
    let token = get_access_token(
        GOOGLE_OAUTH_ACCESS_TOKEN_URL,
        code,
        oauth.client_id.clone(),
        oauth.client_secret.clone(),
        "authorization_code",
        Some(redirect_uri),
    )
    .await?;
    get_user_info(GOOGLE_OAUTH_USER_INFO_URL, token)
        .await
        .map_err(OauthError::UserInfo)?
        .json::<GoogleOauthUserInfo>()
        .await
        .map_err(OauthError::UserInfo)
}
