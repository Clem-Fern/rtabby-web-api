use log::warn;
use serde::{Deserialize};
use crate::models::{user::{User, UserWithoutToken}, config::{UserConfigWithoutDate, NewUserConfig}};
use crate::error::ConfigError;
use std::collections::{HashMap, hash_map::Entry};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub users: Vec<User>,
    pub shared_configs: Vec<UserConfigWithoutDate>,
}

pub fn load_file(file: &str) -> Result<AppConfig, ConfigError> {
    let config_file = std::fs::File::open(file).map_err(ConfigError::Io)?;
    serde_yaml::from_reader(config_file).map_err(ConfigError::Yaml)
}

#[derive(Clone, Debug, Default)]
pub struct MappedAppConfig {
    pub users: HashMap<String, UserWithoutToken>,
    pub shared_configs: HashMap<i32, NewUserConfig>,
}

impl From<AppConfig> for MappedAppConfig {
    fn from(config: AppConfig) -> MappedAppConfig {
        let mut users_map: HashMap<String, UserWithoutToken> = HashMap::new();
        for user in config.users {
            if users_map.contains_key(&user.token) {
                warn!("Config : Skipping user {}, which is not unique ine the configuration", &user.token);
            } else {
                users_map.insert(user.token.clone(), user.clone().into());
            }
        }

        let mut configs_map : HashMap<i32, NewUserConfig> = HashMap::new();
        for config in config.shared_configs {
            if (1..999).contains(&config.id) {
                if let Entry::Vacant(e) = configs_map.entry(config.id) {
                    e.insert(NewUserConfig { name: config.name});
                } else {
                    warn!("Config : Skipping config {}, which is not unique ine the configuration", &config.id);
                }
            } else {
                warn!("Config : Skipping config {}, shared config ID must be between 1 and 999", &config.id);
            }
        }

        MappedAppConfig {
            users: users_map,
            shared_configs: configs_map,
        }

    }
}