use crate::env;

pub fn scheme() -> String {
    let scheme = if env::var(env::ENV_USE_HTTPS).unwrap_or(String::from("0")) == "1" {
        "https"
    }
    else {
        "http"
    }; 
    return String::from(scheme);
}

