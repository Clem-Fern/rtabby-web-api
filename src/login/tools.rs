use crate::login::env;
use crate::env as app_;

pub fn scheme() -> String {
    let scheme = if app_::var(env::ENV_USE_HTTPS).unwrap_or(String::from("0")) == "1" {
        "https"
    }
    else {
        "http"
    }; 
    String::from(scheme)
}

