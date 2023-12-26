use std::collections::HashMap;
use async_trait::async_trait;
use log::info;
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
    
    client.get("https://gitlab.com/api/v4/user")
    .header("Authorization", format!("Bearer {}", token))
    .send()
    .await
}

pub struct GitLab;

#[async_trait]
impl LoginProvider for GitLab {
    fn name(&self) -> String {
        String::from("GitLab")
    }

    fn login_url(&self, host: String, state: String) -> String {
        let client_id = env::var(env::ENV_GITLAB_APP_CLIENT_ID).expect("Missing GITLAB_APP_CLIENT_ID env var");
        format!( "https://gitlab.com/oauth/authorize?client_id={}&redirect_uri={}://{}/login/gitlab/callback&state={}&scope=read_user&response_type=code", client_id, tools::scheme(), host, state)
    }

    async fn user_info(&self, host: String, code: String) -> Result<ThirdPartyUserInfo, Error> {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("code", &code);
        let client_id = env::var(env::ENV_GITLAB_APP_CLIENT_ID).expect("Missing GITLAB_APP_CLIENT_ID env var");
        let client_secret = env::var(env::ENV_GITLAB_APP_CLIENT_SECRET).expect("Missing GITLAB_APP_CLIENT_SECRET env var");
        let grant_type = String::from("authorization_code");
        let redirect_uri = format!("{}://{}/login/gitlab/callback", tools::scheme(), host);
        map.insert("client_id", &client_id);
        map.insert("client_secret", &client_secret);
        map.insert("grant_type", &grant_type);
        map.insert("redirect_uri", &redirect_uri);
        info!("map: {:?}", map);
    
        let res = client.post("https://gitlab.com/oauth/token")
        .header("Accept", "application/json")
        .form(&map)
        .send()
        .await;
        // print res body
        if let Ok(res) = res {
            info!("res: {:?}", res);
            let body = res.json::<Body>().await;
            if let Ok(body) = body {
                info!("body: {:?}", body);
                if let Ok(user_info_resp) = get_user_info(body.access_token).await {
                    let user_info = user_info_resp.json::<UserInfo>().await;
                    if let Ok(user_info) = user_info {
                        Ok(ThirdPartyUserInfo {
                            id: user_info.id.to_string(),
                            name: user_info.name,
                            platform: String::from("github"),
                        })
                    } else {
                        info!("1");
                        Err(actix_web::error::ErrorInternalServerError("Failed to get user info4"))
                    }
                } else {
                    info!("2");
                    Err(actix_web::error::ErrorInternalServerError("Failed to get user info3"))
                }
            } else {
                info!("3");
                Err(actix_web::error::ErrorInternalServerError("Failed to get user info2"))
            }
        } else {
            info!("4");
            if let Err(err) = res {
                info!("err: {:?}", err);
            } 
            Err(actix_web::error::ErrorInternalServerError("Failed to get user info"))
        }
    }
}