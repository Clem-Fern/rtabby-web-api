use async_trait::async_trait;
use crate::{login::providers::{LoginProvider, ThirdPartyUserInfo}, login::tools};
use actix_web::Error;
use crate::login::env;
use crate::env as app_;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    id: String,
    name: String,
}

pub struct Google;

#[async_trait]
impl LoginProvider for Google {
    fn name(&self) -> String {
        String::from("Google")
    }

    fn login_url(&self, host: String, state: String) -> String {
        let client_id = app_::var(env::ENV_GOOGLE_APP_CLIENT_ID).expect("Missing GOOGLE_APP_CLIENT_ID env var");
        let params = vec![
            ("client_id", client_id),
            ("redirect_uri", format!("{}://{}/login/google/callback", tools::scheme(), host)),
            ("state", state),
            ("response_type", "code".to_string()),
            ("scope", "https://www.googleapis.com/auth/userinfo.profile".to_string()),
        ];
        reqwest::Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth", params).unwrap().to_string()
    }

    async fn user_info(&self, host: String, code: String) -> Result<ThirdPartyUserInfo, Error> {
        let client_id = app_::var(env::ENV_GOOGLE_APP_CLIENT_ID).expect("Missing GOOGLE_APP_CLIENT_ID env var");
        let client_secret = app_::var(env::ENV_GOOGLE_APP_CLIENT_SECRET).expect("Missing GOOGLE_APP_CLIENT_SECRET env var");
        let redirect_uri = format!("{}://{}/login/google/callback", tools::scheme(), host);
        let token = Self.get_access_token("https://accounts.google.com/o/oauth2/token".to_string(), code, client_id, client_secret, "authorization_code".to_string(), Some(redirect_uri)).await.unwrap();
        let user_info = Self.get_user_info("https://www.googleapis.com/oauth2/v1/userinfo", token).await.unwrap().json::<UserInfo>().await.unwrap();
        Ok(ThirdPartyUserInfo {
            id: user_info.id.to_string(),
            name: user_info.name,
            platform: self.name().to_lowercase(),
        })
    }
}