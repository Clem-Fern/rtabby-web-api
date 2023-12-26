use std::collections::HashMap;
use async_trait::async_trait;
use crate::routes::login::{LoginProvider, ThirdPartyUserInfo};
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
    .header("X-GitHub-Api-Version", "2022-11-28")
    .header("Accept", "application/vnd.github.v3+json")
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
        let scheme = if env::var(env::ENV_USE_HTTPS).unwrap_or(String::from("0")) == "1" {
            "https"
        }
        else {
            "http"
        }; 
        format!( "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}://{}/login/github/callback&state={}", client_id, scheme, host, state)
    }

    async fn user_id(&self, code: String) -> Result<ThirdPartyUserInfo, Error> {
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
        .await;
        // print res body
        if let Ok(res) = res {
            let body = res.json::<Body>().await;
            if let Ok(body) = body {
                if let Ok(user_info_resp) = get_user_info(body.access_token).await {
                    let user_info = user_info_resp.json::<UserInfo>().await;
                    if let Ok(user_info) = user_info {
                        Ok(ThirdPartyUserInfo {
                            id: user_info.id.to_string(),
                            name: user_info.name,
                            platform: String::from("github"),
                        })
                    } else {
                        Err(actix_web::error::ErrorInternalServerError("Failed to get user info"))
                    }
                } else {
                    Err(actix_web::error::ErrorInternalServerError("Failed to get user info"))
                }
            } else {
                Err(actix_web::error::ErrorInternalServerError("Failed to get user info"))
            }
        } else {
            Err(actix_web::error::ErrorInternalServerError("Failed to get user info"))
        }
    }
}