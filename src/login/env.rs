cfg_if! {
    if #[cfg(feature = "github-login")] {
        pub const ENV_GITHUB_APP_CLIENT_ID: &str = "GITHUB_APP_CLIENT_ID";
        pub const ENV_GITHUB_APP_CLIENT_SECRET: &str = "GITHUB_APP_CLIENT_SECRET";
    }
}

cfg_if! {
    if #[cfg(feature = "google-login")] {
        pub const ENV_GOOGLE_APP_CLIENT_ID: &str = "GOOGLE_APP_CLIENT_ID";
        pub const ENV_GOOGLE_APP_CLIENT_SECRET: &str = "GOOGLE_APP_CLIENT_SECRET";
    }
}

cfg_if! {
    if #[cfg(feature = "gitlab-login")] {
        pub const ENV_GITLAB_APP_CLIENT_ID: &str = "GITLAB_APP_CLIENT_ID";
        pub const ENV_GITLAB_APP_CLIENT_SECRET: &str = "GITLAB_APP_CLIENT_SECRET";
    }
}

cfg_if! {
    if #[cfg(feature = "microsoft-login")] {
        pub const ENV_MICROSOFT_APP_CLIENT_ID: &str = "MICROSOFT_APP_CLIENT_ID";
        pub const ENV_MICROSOFT_APP_CLIENT_SECRET: &str = "MICROSOFT_APP_CLIENT_SECRET";
    }
}

pub const ENV_STATIC_FILES_BASE_DIR: &str = "STATIC_FILES_BASE_DIR";
pub const ENV_USE_HTTPS: &str = "USE_HTTPS";

use crate::env as app_;

pub fn static_files_base_dir() -> String {
    app_::var(ENV_STATIC_FILES_BASE_DIR).unwrap_or("./web/".to_string())
}