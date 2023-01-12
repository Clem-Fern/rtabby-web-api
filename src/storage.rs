extern crate diesel;
use std::error::Error;

use diesel::{prelude::*, sqlite::Sqlite};

extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use diesel::r2d2::{Pool, ConnectionManager};

use crate::env;
use crate::error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct Storage {
    url: String,
}

impl Storage {
    pub fn new() -> Self {
        let database_url = env::var(env::ENV_DATABASE_URL).unwrap_or("file:data.db".to_string());
        Storage { url: database_url }
    }

    pub fn url(self) -> String {
        self.url
    }

    pub fn init(self) -> Result<(), error::StorageInitializationError> {
        let mut conn = establish_connection(self.url().as_str());
        run_migrations(&mut conn)?;
        Ok(())
    }

    pub fn pool(self) -> Result<SqlitePool, error::StorageInitializationError> {
        let pool = Pool::new(ConnectionManager::new(self.url()))?;
        Ok(pool)
    }
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if connection.has_pending_migration(MIGRATIONS)? {
        info!("Running pending migrations.");
        connection.run_pending_migrations(MIGRATIONS)?;
    }

    Ok(())
}

pub fn establish_connection(url: &str) -> SqliteConnection {
    SqliteConnection::establish(url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}
