#[cfg(feature = "github-login")]
pub mod github;
#[cfg(feature = "gitlab-login")]
pub mod gitlab;
#[cfg(feature = "google-login")]
pub mod google;
#[cfg(feature = "microsoft-login")]
pub mod microsoft;


use actix_web::Error;
use log::error;
use serde::Deserialize;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Body {
    access_token: String,
}

#[async_trait]
pub trait LoginProvider {
    fn name(&self) -> String;
    fn login_url(&self, host: String, state: String) -> String;
    async fn user_info(&self, host: String, code: String) -> Result<ThirdPartyUserInfo, Error>;
    
    async fn get_user_info(
        &self,
        url: &str,
        token: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        
        client.get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "actix-web/3.3.2")
        .send()
        .await
    }

    async fn get_access_token(&self, 
        url: String, 
        code: String, 
        client_id: String, 
        client_secret: String, 
        grant_type: String,
        redirect_uri: Option<String>) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("code", code);
        map.insert("client_id", client_id);
        map.insert("client_secret", client_secret);
        map.insert("grant_type", grant_type);
        if let Some(redirect_uri) = redirect_uri {
            map.insert("redirect_uri", redirect_uri);
        }
    
        let res = client.post(url)
        .form(&map)
        .header("Accept", "application/json")
        .send()
        .await.map_err(|e| {
            error!("Error while getting user info: {:?}", e);
            e
        });
    
        let token = res.unwrap().json::<Body>().await.unwrap().access_token;
        Ok(token)
    }
}

#[derive(Debug, Deserialize)]
pub struct ThirdPartyUserInfo {
    pub id: String,
    pub name: String,
    pub platform: String,
}