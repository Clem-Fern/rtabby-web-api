extern crate diesel;
use std::error::Error;

use diesel::prelude::*;
#[cfg(feature = "mysql")]
use diesel::mysql::Mysql;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;


extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use diesel::r2d2::{Pool, ConnectionManager};

use crate::app_config::MappedAppConfig;
use crate::env;
use crate::error;

#[cfg(feature = "mysql")]
const MYSQL_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
#[cfg(feature = "sqlite")]
const SQLITE_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations_sqlite");

#[derive(diesel::MultiConnection)]
pub enum DbConnection {
    #[cfg(feature = "mysql")]
    Mysql(diesel::MysqlConnection),
    #[cfg(feature = "sqlite")]
    Sqlite(diesel::SqliteConnection),
}

pub type DbPool = Pool<ConnectionManager<DbConnection>>;



#[derive(Clone)]
pub struct Storage {
    url: String,
}

impl Storage {
    pub fn new() -> Self {
        let database_url = env::var(env::ENV_DATABASE_URL).unwrap_or("mysql://tabby:tabby@db/tabby".to_string());
        Storage { url: database_url }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn init(&self) -> Result<(), error::StorageError> {
        let mut conn = establish_connection(self.url().as_str())?;

        // RUN PENDING MIGRATIONS
        match conn {
            #[cfg(feature = "mysql")]
            DbConnection::Mysql(ref mut conn) => {
                run_mysql_migrations(conn)?;
            },
            #[cfg(feature = "sqlite")]
            DbConnection::Sqlite(ref mut conn) => {
                run_sqlite_migrations(conn)?;
            }            
        }

        Ok(())
    }

    pub fn cleanup(&self, app_config: &MappedAppConfig) -> Result<(), error::StorageError> {
        let mut conn = establish_connection(self.url().as_str())?;

        use crate::schema::configs::dsl::*;

        diesel::delete(configs.filter(user.ne_all(app_config.users.keys()))).execute(&mut conn)?;

        Ok(())
    }

    pub fn pool(&self) -> Result<DbPool, error::StorageError> {
        let pool = Pool::new(ConnectionManager::new(self.url().clone()))?;
        
        Ok(pool)
    }
}

#[cfg(feature = "mysql")]
fn run_mysql_migrations(
    connection: &mut impl MigrationHarness<Mysql>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if connection.has_pending_migration(MYSQL_MIGRATIONS)? {
        info!("Running pending migrations.");
        connection.run_pending_migrations(MYSQL_MIGRATIONS)?;
    }

    Ok(())
}

#[cfg(feature = "sqlite")]
fn run_sqlite_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if connection.has_pending_migration(SQLITE_MIGRATIONS)? {
        info!("Running pending migrations.");
        connection.run_pending_migrations(SQLITE_MIGRATIONS)?;
    }

    Ok(())
}

pub fn establish_connection(url: &str) -> Result<DbConnection, diesel::ConnectionError> {
    #[cfg(feature = "mysql")]
    if url.starts_with("mysql://") {
        return Ok(DbConnection::Mysql(MysqlConnection::establish(url)?));
    }
    
    #[cfg(feature = "sqlite")]
    if url.starts_with("sqlite://") {
        return Ok(DbConnection::Sqlite(SqliteConnection::establish(url)?));
    }

    DbConnection::establish(url)
}
