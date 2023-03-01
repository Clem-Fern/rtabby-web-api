pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub mod config;
pub mod user;
pub mod user_config;