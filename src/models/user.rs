use serde::{de, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct LocalUser {
    pub name: String,
    #[serde(deserialize_with = "uuid_validator")]
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserWithoutToken {
    pub name: String,
}

impl From<LocalUser> for UserWithoutToken {
    fn from(user: LocalUser) -> Self {
        UserWithoutToken {
            name: user.name,
        }
    }
}

pub fn uuid_validator<'de, D>(d: D) -> Result<String, D::Error>
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
