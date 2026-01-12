use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ProviderError {
    NotFound(String),
}

impl std::error::Error for ProviderError {}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NotFound(ref provider) => {
                write!(f, "Specified provider does not exist: {provider}")
            }
        }
    }
}

#[derive(Debug)]
pub enum OauthError {
    UserInfo(reqwest::Error),
    AccessToken(reqwest::Error),
    #[cfg(feature = "oidc-login")]
    OIDCConfiguration(reqwest::Error),
}

impl error::Error for OauthError {}

impl fmt::Display for OauthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::UserInfo(ref err) => write!(f, "Unable to retreive OAuth user info: {err}"),
            Self::AccessToken(ref err) => {
                write!(f, "Unable to retreive OAuth user access token: {err}")
            }
            #[cfg(feature = "oidc-login")]
            Self::OIDCConfiguration(ref err) => {
                write!(f, "Unable to retreive OIDC configuration: {err}")
            }
        }
    }
}
