#[cfg(feature = "github-login")]
pub mod github;
#[cfg(feature = "gitlab-login")]
pub mod gitlab;
#[cfg(feature = "google-login")]
pub mod google;
#[cfg(feature = "microsoft-login")]
pub mod microsoft;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use super::error::OauthError;
use super::tools;

#[derive(Clone, Debug)]
pub struct OauthInfo {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct OauthUserInfo<I = String, N = String> {
    id: I,
    name: N,
}

// Gitlab / Github
impl From<OauthUserInfo<i32, String>> for OauthUserInfo {
    fn from(val: OauthUserInfo<i32, String>) -> Self {
        OauthUserInfo {
            id: format!("{}", val.id),
            name: val.name,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Platform {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug)]
pub enum Provider {
    #[cfg(feature = "github-login")]
    Github(OauthInfo),
    #[cfg(feature = "gitlab-login")]
    Gitlab(OauthInfo),
    #[cfg(feature = "google-login")]
    Google(OauthInfo),
    #[cfg(feature = "microsoft-login")]
    Microsoft(OauthInfo),
}

impl Provider {
    pub fn name(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn get_oauth_info(&self) -> OauthInfo {
        match self {
            #[cfg(feature = "github-login")]
            Self::Github(oauth) => oauth.clone(),
            #[cfg(feature = "gitlab-login")]
            Self::Gitlab(oauth) => oauth.clone(),
            #[cfg(feature = "google-login")]
            Self::Google(oauth) => oauth.clone(),
            #[cfg(feature = "microsoft-login")]
            Self::Microsoft(oauth) => oauth.clone(),
        }
    }

    fn get_login_url_params(&self, host: String, state: String) -> Vec<(&str, String)> {
        let mut params = vec![
            ("client_id", self.get_oauth_info().client_id),
            ("state", state),
            ("redirect_uri", format!("{}://{}/login/{}/callback", tools::scheme(), host, self.name())),
        ];

        if !matches!(self, Self::Github(_)) {
            params.push(("response_type", "code".to_string()));
        }

        match self {
            Self::Gitlab(_) => {
                params.push(("scope", "read_user".to_string()));
            }
            Self::Google(_) => {
                params.push(("scope", "https://www.googleapis.com/auth/userinfo.profile".to_string()));
            },
            Self::Microsoft(_) => {
                params.push(("scope", "https://graph.microsoft.com/User.Read".to_string()));
            },
            _ => {},
        }

        params
    }

    pub fn get_login_url(&self, host: String, state: String) -> String {

        let params = self.get_login_url_params(host, state);

        let oauth_url = match self {
            #[cfg(feature = "github-login")]
            Self::Github(_) => github::GITHUB_OAUTH_AUTHORIZE_URL,
            #[cfg(feature = "gitlab-login")]
            Self::Gitlab(_) => gitlab::GITLAB_OAUTH_AUTHORIZE_URL,
            #[cfg(feature = "google-login")]
            Self::Google(_) => google::GOOGLE_OAUTH_AUTHORIZE_URL,
            #[cfg(feature = "microsoft-login")]
            Self::Microsoft(_) => microsoft::MICROSOFT_OAUTH_AUTHORIZE_URL,
        };

        reqwest::Url::parse_with_params(oauth_url, params).unwrap().to_string()
    }

    pub async fn get_user_info(&self, host: String, token: String) -> Result<ThirdPartyUserInfo, OauthError> {
        let user_info = match self {
            #[cfg(feature = "github-login")]
            Self::Github(oauth) => github::user_info(oauth, host).await?.into(),
            #[cfg(feature = "gitlab-login")]
            Self::Gitlab(oauth) => gitlab::user_info(oauth, host, token).await?.into(),
            #[cfg(feature = "google-login")]
            Self::Google(oauth) => google::user_info(oauth, host, token).await?,
            #[cfg(feature = "microsoft-login")]
            Self::Microsoft(oauth) => microsoft::user_info(oauth, host, token).await?.into(),
        };

        Ok(ThirdPartyUserInfo {
            id: user_info.id,
            name: user_info.name,
            platform: self.name(),
        })
    }

}

impl From<Provider> for Platform {
    fn from(provider: Provider) -> Platform {
        Platform {
            name: provider.to_string(),
            url: format!("login/{}", provider.name()),
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "github-login")]
            Self::Github(_) => write!(f, "Github"),
            #[cfg(feature = "gitlab-login")]
            Self::Gitlab(_) => write!(f, "Gitlab"),
            #[cfg(feature = "google-login")]
            Self::Google(_) => write!(f, "Google"),
            #[cfg(feature = "microsoft-login")]
            Self::Microsoft(_) => write!(f, "Microsoft"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Body {
    access_token: String,
}

pub async fn get_user_info(url: &str, token: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "actix-web/3.3.2")
        .send()
        .await
}

async fn get_access_token(
    url: &str,
    code: String,
    client_id: String,
    client_secret: String,
    grant_type: &str,
    redirect_uri: Option<String>,
) -> Result<String, OauthError> {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("code", code);
    map.insert("client_id", client_id);
    map.insert("client_secret", client_secret);
    map.insert("grant_type", String::from(grant_type));
    if let Some(redirect_uri) = redirect_uri {
        map.insert("redirect_uri", redirect_uri);
    }

    let res = client
        .post(url)
        .form(&map)
        .header("Accept", "application/json")
        .send()
        .await.map_err(OauthError::AccessToken)?;

    Ok(res.json::<Body>().await.map_err(OauthError::AccessToken)?.access_token)
}

#[derive(Debug, Deserialize)]
pub struct ThirdPartyUserInfo {
    pub id: String,
    pub name: String,
    pub platform: String,
}
