use serde::{de, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "uuid_validator")]
    pub token: String,
    #[serde(default)]
    pub shared_configs: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserWithoutSharedConfig {
    #[serde(deserialize_with = "uuid_validator")]
    pub token: String,
}

impl From<User> for UserWithoutSharedConfig {
    fn from(user: User) -> Self {
        UserWithoutSharedConfig { token: user.token }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserWithoutToken {
    pub shared_configs: Vec<i32>,
}

impl From<User> for UserWithoutToken {
    fn from(user: User) -> Self {
        UserWithoutToken {
            shared_configs: user.shared_configs,
        }
    }
}

fn uuid_validator<'de, D>(d: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = String::deserialize(d)?;

    if Uuid::parse_str(&value).is_err() {
        return Err(de::Error::invalid_value(
            de::Unexpected::Str(&value),
            &"a valid UUIDv4",
        ));
    }

    Ok(value)
}
