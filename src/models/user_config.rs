use serde::{Serialize, Deserialize};
use diesel::{Queryable, Identifiable, MysqlConnection, QueryDsl, RunQueryDsl, ExpressionMethods, OptionalExtension, BoolExpressionMethods, Associations};

use super::DbError;

use crate::schema::user_configs;
use crate::schema::user_configs::dsl::user_configs as all_user_configs;

use crate::models::config::Config;

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[diesel(table_name = user_configs)]
#[diesel(primary_key(config_id, user))]
#[diesel(belongs_to(Config))]
pub struct UserConfig {
    pub config_id: i32,

    pub user: String,

    #[serde(default)]
    pub content: String,

}

impl UserConfig {
    pub fn get_user_config_by_config_id_and_user(conn: &mut MysqlConnection, id: i32, user_id: &str) -> Result<Option<UserConfig>, DbError> {

        use crate::schema::user_configs::dsl::*;

        Ok(all_user_configs.filter(config_id.eq(id).and(user.eq(user_id))).first::<UserConfig>(conn).optional()?)
    }
}