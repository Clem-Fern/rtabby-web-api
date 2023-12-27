use std::collections::HashMap;
use async_trait::async_trait;
use log::error;
use crate::{routes::login::{LoginProvider, ThirdPartyUserInfo}, login::tools};
use actix_web::Error;
use crate::env;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Body {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    id: i32,
    name: String,
}

async fn get_user_info(
    token: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    
    client.get("https://api.github.com/user")
    .header("Authorization", format!("Bearer {}", token))
    .header("User-Agent", "actix-web/3.3.2")
    .send()
    .await
}

pub struct Github;

#[async_trait]
impl LoginProvider for Github {
    fn name(&self) -> String {
        String::from("Github")
    }

    fn login_url(&self, host: String, state: String) -> String {
        let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
        format!( "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}://{}/login/github/callback&state={}", client_id, tools::scheme(), host, state)
    }

    async fn user_info(&self, _host: String, code: String) -> Result<ThirdPartyUserInfo, Error> {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("code", &code);
        let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
        let client_secret = env::var(env::ENV_GITHUB_APP_CLIENT_SECRET).expect("Missing GITHUB_APP_CLIENT_SECRET env var");
        map.insert("client_id", &client_id);
        map.insert("client_secret", &client_secret);
    
        let res = client.post("https://github.com/login/oauth/access_token")
        .json(&map)
        .header("Accept", "application/json")
        .send()
        .await.map_err(|e| {
            error!("Error while getting user info from GitHub: {:?}", e);
            e
        });
    
        let token = res.unwrap().json::<Body>().await.unwrap().access_token;
        let user_info = get_user_info(token).await.unwrap().json::<UserInfo>().await.unwrap();
        Ok(ThirdPartyUserInfo {
            id: user_info.id.to_string(),
            name: user_info.name,
            platform: self.name().to_lowercase(),
        })
    }
}