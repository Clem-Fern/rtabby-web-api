use crate::login::error::OauthError;
use crate::login::providers::{get_user_info, get_access_token, OauthInfo, OauthUserInfo};
use crate::login::tools;
use serde::Deserialize;

pub mod env {
    pub const ENV_MICROSOFT_APP_CLIENT_ID: &str = "MICROSOFT_APP_CLIENT_ID";
    pub const ENV_MICROSOFT_APP_CLIENT_SECRET: &str = "MICROSOFT_APP_CLIENT_SECRET";
}

pub const MICROSOFT_OAUTH_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
pub const MICROSOFT_OAUTH_ACCESS_TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
pub const MICROSOFT_OAUTH_USER_INFO_URL: &str = "https://graph.microsoft.com/v1.0/me";

#[derive(Debug, Deserialize)]
pub struct MicrosoftOauthUserInfo {
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
}


impl From<MicrosoftOauthUserInfo> for OauthUserInfo {
    fn from(val: MicrosoftOauthUserInfo) -> Self {
        OauthUserInfo {
            id: val.id,
            name: val.display_name,
        }
    }
}

pub async fn user_info(oauth: &OauthInfo, host: String, code: String) -> Result<MicrosoftOauthUserInfo, OauthError> {
    let redirect_uri = format!("{}://{}/login/microsoft/callback", tools::scheme(), host);
    let token = get_access_token(MICROSOFT_OAUTH_ACCESS_TOKEN_URL, code, oauth.client_id.clone(), oauth.client_secret.clone(), "authorization_code", Some(redirect_uri)).await?;
    get_user_info(MICROSOFT_OAUTH_USER_INFO_URL, token).await.map_err(OauthError::UserInfo)?.json::<MicrosoftOauthUserInfo>().await.map_err(OauthError::UserInfo)
}
