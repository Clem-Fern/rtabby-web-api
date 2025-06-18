use crate::error::ConfigError;
use crate::models::user::{LocalUser, UserWithoutToken};
use log::warn;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub users: Vec<LocalUser>,
}

pub fn load_file(file: &str) -> Result<AppConfig, ConfigError> {
    let config_file = std::fs::File::open(file).map_err(ConfigError::Io)?;
    serde_yaml::from_reader(config_file).map_err(ConfigError::Yaml)
}

pub fn create_config_file_if_not_exist(file: &str) -> Result<(), ConfigError> {
    let path = Path::new(file);
    if path.exists() {
        Ok(())
    } else {
        let mut config = File::create(path)?;
        write!(config, include_str!("../users.exemple.yml"))?;
        Err(ConfigError::NoConfig(String::from(file)))
    }
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
                warn!(
                    "Config : Skipping user {}, which is not unique in the configuration",
                    &user.token
                );
            } else {
                users_map.insert(user.token.clone(), user.clone().into());
            }
        }

        MappedAppConfig { users: users_map }
    }
}
