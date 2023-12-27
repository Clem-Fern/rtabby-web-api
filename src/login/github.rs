use async_trait::async_trait;
use crate::{login::provider::{LoginProvider, ThirdPartyUserInfo}, login::tools};
use actix_web::Error;
use crate::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    id: i32,
    name: String,
}

pub struct Github;

#[async_trait]
impl LoginProvider for Github {
    fn name(&self) -> String {
        String::from("Github")
    }

    fn login_url(&self, host: String, state: String) -> String {
        let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
        let params = vec![
            ("client_id", client_id),
            ("state", state),
            ("redirect_uri", format!("{}://{}/login/github/callback", tools::scheme(), host)),
        ];
        reqwest::Url::parse_with_params("https://github.com/login/oauth/authorize", params).unwrap().to_string()
    }

    async fn user_info(&self, _host: String, code: String) -> Result<ThirdPartyUserInfo, Error> {
        let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
        let client_secret = env::var(env::ENV_GITHUB_APP_CLIENT_SECRET).expect("Missing GITHUB_APP_CLIENT_SECRET env var");
        let token = Self.get_access_token("https://github.com/login/oauth/access_token".to_string(), code, client_id, client_secret, "authorization_code".to_string(), None).await.unwrap();
        let user_info = Self.get_user_info("https://api.github.com/user", token).await.unwrap().json::<UserInfo>().await.unwrap();
        Ok(ThirdPartyUserInfo {
            id: user_info.id.to_string(),
            name: user_info.name,
            platform: self.name().to_lowercase(),
        })
    }
}