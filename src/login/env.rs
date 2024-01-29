pub const ENV_STATIC_FILES_BASE_DIR: &str = "STATIC_FILES_BASE_DIR";
pub const ENV_USE_HTTPS: &str = "USE_HTTPS";

use crate::env as app_env;

pub fn static_files_base_dir() -> String {
    app_env::var(ENV_STATIC_FILES_BASE_DIR).unwrap_or("./web/".to_string())
}