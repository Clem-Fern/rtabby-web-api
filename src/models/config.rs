use diesel::sql_types::{Integer, VarChar};
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, Identifiable, MysqlConnection, QueryDsl, RunQueryDsl, ExpressionMethods, OptionalExtension, BoolExpressionMethods, NullableExpressionMethods, AsChangeset, sql_query};
use chrono::{NaiveDateTime, Utc};

use super::DbError;

pub const MAX_SHARED_CONFIG_ID: i32 = 999;

use crate::schema::configs;
use crate::schema::configs::dsl::configs as all_configs;

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[diesel(table_name = configs)]
#[diesel(primary_key(id))]
pub struct Config {
    pub id: i32,
    pub name: String,

    #[serde(default)]
    pub user: Option<String>,

    #[serde(default)]
    pub content: String,
    
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl Config {

    pub fn get_all_config_by_user(conn: &mut MysqlConnection, user_id: &str) -> Result<Vec<Config>, DbError> {

        use crate::schema::configs::dsl::*;

        let list = configs.filter(user.eq(user_id)).load::<Config>(conn)?;

        Ok(list)
    }
    
    pub fn insert_new_user_config(conn: &mut MysqlConnection, new_config: NewConfigWithUser) -> Result<(), DbError> {

        diesel::insert_into(all_configs).values(&new_config).execute(conn)?;

        Ok(())
    }

    pub fn insert_new_user_config_or_update(conn: &mut MysqlConnection, config: ConfigWithoutDate) -> Result<(), DbError> {

        let query = sql_query("INSERT INTO configs(id, name) VALUES (?,?) ON DUPLICATE KEY UPDATE name=?;");
        
        query.bind::<Integer, _>(config.id).bind::<VarChar, _>(config.name.clone()).bind::<VarChar, _>(config.name).execute(conn)?;

        Ok(())
    }

    pub fn get_config_by_id_and_user(conn: &mut MysqlConnection, config_id: i32, user_id: &str) -> Result<Option<Config>, DbError> {

        use crate::schema::configs::dsl::*;

        Ok(all_configs.filter(id.eq(config_id).and(user.nullable().eq(user_id))).first::<Config>(conn).optional()?)
    }

    pub fn get_shared_config_by_id(conn: &mut MysqlConnection, config_id: i32) -> Result<Option<Config>, DbError> {

        use crate::schema::configs::dsl::*;

        Ok(all_configs.filter(id.eq(config_id).and(user.is_null())).first::<Config>(conn).optional()?)
    }

    pub fn update_user_config_content(conn: &mut MysqlConnection, config: Config, new_content: &str) -> Result<(), DbError> {

        use crate::schema::configs::dsl::*;

        diesel::update(&config).set((
            content.eq(new_content),
            modified_at.eq(Utc::now().naive_utc())
        )).execute(conn)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = configs)]
#[diesel(primary_key(id))]
pub struct ConfigWithoutDate {
    pub id: i32,
    pub name: String,

    #[serde(default)]
    pub user: Option<String>,

    #[serde(default)]
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = configs)]
pub struct NewConfig {
    pub name: String,
}

impl NewConfig {
    pub fn into_new_user_config_with_user(self, user: String) -> NewConfigWithUser {
        NewConfigWithUser { name: self.name, user }
    }

    pub fn into_user_config_without_date(self, id: i32) -> ConfigWithoutDate {
        ConfigWithoutDate { id, name: self.name, user: Option::default(), content: String::default() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = configs)]
pub struct NewConfigWithUser {
    name: String,
    #[serde(default)]
    pub user: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = configs)]
pub struct UpdateConfig {
    pub content: String,
}