use log::warn;
use serde::{Deserialize};
use crate::models::{user::{User, UserWithoutToken}, config::UserConfigWithoutDate};
use crate::error::ConfigError;
use std::collections::{HashMap, hash_map::Entry};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub users: Vec<User>,
    pub shared_configs: Vec<UserConfigWithoutDate>,
}

pub fn load_file(file: &str) -> Result<Config, ConfigError> {
    let config_file = std::fs::File::open(file).map_err(ConfigError::Io)?;
    serde_yaml::from_reader(config_file).map_err(ConfigError::Yaml)
}

#[derive(Clone, Debug, Default)]
pub struct MappedConfig {
    pub users: HashMap<String, UserWithoutToken>,
    pub shared_configs: HashMap<i32, UserConfigWithoutDate>,
}

impl From<Config> for MappedConfig {
    fn from(config: Config) -> MappedConfig {
        let mut users_map: HashMap<String, UserWithoutToken> = HashMap::new();
        for user in config.users {
            if users_map.contains_key(&user.token) {
                warn!("Config : Skipping user {}, which is not unique ine the configuration", &user.token);
            } else {
                users_map.insert(user.token.clone(), user.clone().into());
            }
        }

        let mut configs_map : HashMap<i32, UserConfigWithoutDate> = HashMap::new();
        for config in config.shared_configs {
            if (1..999).contains(&config.id) {
                if let Entry::Vacant(e) = configs_map.entry(config.id) {
                    e.insert(config.clone());
                } else {
                    warn!("Config : Skipping config {}, which is not unique ine the configuration", &config.id);
                }
            } else {
                warn!("Config : Skipping config {}, shared config ID must be between 1 and 999", &config.id);
            }
        }

        MappedConfig {
            users: users_map,
            shared_configs: configs_map,
        }

    }
}