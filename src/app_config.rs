use log::warn;
use serde::Deserialize;
use crate::models::user::{User, UserWithoutToken};
use crate::error::ConfigError;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub users: Vec<User>,
}

pub fn load_file(file: &str) -> Result<AppConfig, ConfigError> {
    let config_file = std::fs::File::open(file).map_err(ConfigError::Io)?;
    serde_yaml::from_reader(config_file).map_err(ConfigError::Yaml)
}

#[derive(Clone, Debug, Default)]
pub struct MappedAppConfig {
    pub users: HashMap<String, UserWithoutToken>,
}

impl From<AppConfig> for MappedAppConfig {
    fn from(config: AppConfig) -> MappedAppConfig {
        let mut users_map: HashMap<String, UserWithoutToken> = HashMap::new();
        for user in config.users {
            if users_map.contains_key(&user.token) {
                warn!("Config : Skipping user {}, which is not unique in the configuration", &user.token);
            } else {
                users_map.insert(user.token.clone(), user.clone().into());
            }
        }

        MappedAppConfig {
            users: users_map,
        }

    }
}