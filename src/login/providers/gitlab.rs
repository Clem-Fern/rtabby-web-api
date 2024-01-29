use async_trait::async_trait;
use crate::{login::providers::{LoginProvider, ThirdPartyUserInfo}, login::tools};
use actix_web::Error;
use crate::env as app_env;
use serde::Deserialize;

pub mod env {
    pub const ENV_GITLAB_APP_CLIENT_ID: &str = "GITLAB_APP_CLIENT_ID";
    pub const ENV_GITLAB_APP_CLIENT_SECRET: &str = "GITLAB_APP_CLIENT_SECRET";
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    id: i32,
    name: String,
}

pub struct GitLab;

#[async_trait]
impl LoginProvider for GitLab {
    fn name(&self) -> String {
        String::from("GitLab")
    }

    fn login_url(&self, host: String, state: String) -> String {
        let client_id = app_env::var(env::ENV_GITLAB_APP_CLIENT_ID).expect("Missing GITLAB_APP_CLIENT_ID env var");
        let params = vec![
            ("client_id", client_id),
            ("state", state),
            ("redirect_uri", format!("{}://{}/login/gitlab/callback", tools::scheme(), host)),
            ("response_type", "code".to_string()),
            ("scope", "read_user".to_string()),
        ];
        reqwest::Url::parse_with_params("https://gitlab.com/oauth/authorize", params).unwrap().to_string()
    }

    async fn user_info(&self, host: String, code: String) -> Result<ThirdPartyUserInfo, Error> {
        let client_id = app_env::var(env::ENV_GITLAB_APP_CLIENT_ID).expect("Missing GITLAB_APP_CLIENT_ID env var");
        let client_secret = app_env::var(env::ENV_GITLAB_APP_CLIENT_SECRET).expect("Missing GITLAB_APP_CLIENT_SECRET env var");
        let redirect_uri = format!("{}://{}/login/gitlab/callback", tools::scheme(), host);
        let token = Self.get_access_token("https://gitlab.com/oauth/token".to_string(), code, client_id, client_secret, "authorization_code".to_string(), Some(redirect_uri)).await.unwrap();
        let user_info = Self.get_user_info("https://gitlab.com/api/v4/user", token).await.unwrap().json::<UserInfo>().await.unwrap();
        Ok(ThirdPartyUserInfo {
            id: user_info.id.to_string(),
            name: user_info.name,
            platform: self.name().to_lowercase(),
        })
    }
}