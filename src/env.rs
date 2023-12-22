#[cfg(feature = "dotenv")]
extern crate dotenvy;

pub const ENV_CONFIG_FILE: &str = "CONFIG_FILE";
pub const ENV_BIND_ADDR: &str = "BIND_ADDR";
pub const ENV_BIND_PORT: &str = "BIND_PORT";
pub const ENV_SSL_CERTIFICATE: &str = "SSL_CERTIFICATE";
pub const ENV_SSL_CERTIFICATE_KEY: &str = "SSL_CERTIFICATE_KEY";
pub const ENV_CLEANUP_USERS: &str = "CLEANUP_USERS";

pub const ENV_DATABASE_URL: &str = "DATABASE_URL";

pub const ENV_GITHUB_APP_CLIENT_ID: &str = "GITHUB_APP_CLIENT_ID";
pub const ENV_GITHUB_APP_CLIENT_SECRET: &str = "GITHUB_APP_CLIENT_SECRET";

pub fn init() {
    // LOAD ENV VAR from .env if dotenv feature is enable
    #[cfg(feature = "dotenv")]
    {
        dotenvy::dotenv().expect(".env file not found");
    }
}

pub use std::env::*;


