use serde::{Deserialize, Serialize};
use crate::models::DbError;
use crate::models::user::uuid_validator;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, Identifiable, Insertable,
    OptionalExtension, QueryDsl, Queryable,
    RunQueryDsl,
};
use crate::storage::DbConnection;
use chrono::NaiveDateTime;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub user_id: String,
    pub platform: String,
    #[serde(deserialize_with = "uuid_validator")]
    pub token: String,

    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub user_id: String,
    pub platform: String,
    pub token: String,
}

impl User {

    pub fn insert_new_user_config(
        conn: &mut DbConnection,
        new_user: NewUser,
    ) -> Result<(), DbError> {

        match conn {
            #[cfg(feature = "mysql")]
            DbConnection::Mysql(ref mut conn) => {
                diesel::insert_into(all_users)
                .values(&new_user)
                .execute(conn)?;
            },
            #[cfg(feature = "sqlite")]
            DbConnection::Sqlite(ref mut conn) => {
                diesel::insert_into(all_users)
                .values(&new_user)
                .execute(conn)?;
            }
        }

        Ok(())
    }


    pub fn get_user_by_token(
        conn: &mut DbConnection,
        in_token: &str,
    ) -> Result<Option<User>, DbError> {
        use crate::schema::users::dsl::*;

        Ok(all_users
            .filter(token.eq(in_token))
            .first::<User>(conn)
            .optional()?)
    }

    pub fn get_user(
        conn: &mut DbConnection,
        in_user_id: &str,
        in_platform: &str,
    ) -> Result<Option<User>, DbError> {
        use crate::schema::users::dsl::*;

        Ok(all_users
            .filter(platform.eq(in_platform).and(user_id.eq(in_user_id)))
            .first::<User>(conn)
            .optional()?)
    }

    pub fn delete_user(conn: &mut DbConnection, user: User) -> Result<(), DbError> {
        diesel::delete(&user).execute(conn)?;

        Ok(())
    }
}
