extern crate env_logger;
extern crate log;
use log::{info, error, warn};

use std::error::Error;

mod app_config;
mod env;
mod error;
use app_config::{AppConfig, MappedAppConfig};
mod storage;
use storage::Storage;

mod auth;
mod models;
mod routes;
mod schema;

extern crate serde_yaml;

extern crate actix_web;
use actix_web::{middleware, web, App, HttpServer};

#[cfg(feature = "third-party-login")]
mod login;

extern crate actix_web_httpauth;
use actix_web_httpauth::middleware::HttpAuthentication;

mod tls;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // third-party-login should only be enable by one of the features below (github-login, gitlab-login, google-login, microsoft-login)
    #[cfg(feature = "third-party-login")]
    {
        #[cfg(not(any(feature = "github-login", feature = "gitlab-login", feature = "google-login", feature = "microsoft-login")))]
        {
            compile_error!("You must enable at least one login provider feature to use the login feature.");
        }
    }

    // INITIALIZE LOGGING
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        },
    ))
    .init();
    info!("Running v{}", env!("CARGO_PKG_VERSION"));

    // LOAD ENV VAR from .env if dotenv feature is enable
    env::init();

    match run_app().await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            Err(err)
        }
    }

}

async fn run_app() -> Result<(), Box<dyn Error>> {
    // LOAD CONFIG FILE
    let config_file_name = env::var(env::ENV_CONFIG_FILE).unwrap_or(String::from("users.yml"));

    // Check if the config file already exist, else create one and exit
    app_config::create_config_file_if_not_exist(&config_file_name)?;

    let config: AppConfig = app_config::load_file(&config_file_name)?;
    let config: MappedAppConfig = config.into();

    info!("{} loaded => {} users found", config_file_name, config.users.len());

    // INIT DATABASE STORAGE
    let storage: Storage = Storage::new();
    storage.init()?;
    
    // storage clean up on start
    if env::var(env::ENV_CLEANUP_USERS).unwrap_or(String::from("false")).to_lowercase().parse().unwrap_or(false) {
        warn!("Cleaning up old user configurations from storage.");
        storage.cleanup(&config)?;
    }

    #[cfg(feature = "third-party-login")]
    let providers_config: login::ProvidersConfig = login::get_provider_config();

    #[cfg(feature = "third-party-login")]
    info!("Third party login enabled: {} providers found.", providers_config.available_providers.len());

    let pool = storage.pool()?;
    let mut server = HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(config.clone())) // App Config Data
            .app_data(web::Data::new(pool.clone())) // Database Pool Data
            .wrap(middleware::Logger::default().log_target(env!("CARGO_PKG_NAME").to_string()))
            .configure(api_v1_config);

        #[cfg(feature = "third-party-login")]
        if !providers_config.available_providers.is_empty() {
            return app.app_data(web::Data::new(providers_config.clone()))
                .configure(login::services::login_config);
        }
        
        #[allow(clippy::let_and_return)]
        app

    });

    // socket var
    let bind_addr = env::var(env::ENV_BIND_ADDR).unwrap_or(String::from("0.0.0.0"));
    let bind_port = env::var(env::ENV_BIND_PORT).unwrap_or(String::from("8080"));

    if env::var(env::ENV_SSL_CERTIFICATE).is_ok() || env::var(env::ENV_SSL_CERTIFICATE_KEY).is_ok()
    {

        let ssl_certificate =
            env::var(env::ENV_SSL_CERTIFICATE).expect("Missing SSL_CERTIFICATE env var");
        let ssl_certificate_key =
            env::var(env::ENV_SSL_CERTIFICATE_KEY).expect("Missing SSL_CERTIFICATE_KEY env var");

        let config = tls::TLSConfigBuilder::new()
            .load_certs(&ssl_certificate)?
            .load_private_key(&ssl_certificate_key)?
            .build()?;

        info!("Binding HTTPS Listener on {bind_addr}:{bind_port}");
        server = server.bind_rustls_021(format!("{bind_addr}:{bind_port}"), config)?;
    } else {
        info!("Binding HTTP Listener on {bind_addr}:{bind_port}");
        server = server.bind(format!("{bind_addr}:{bind_port}"))?;
    }

    info!("Starting HTTP Listener on {bind_addr}:{bind_port}");
    server.run().await?;
    Ok(())
}

// configure service & route for api v1
fn api_v1_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/1")
                .configure(routes::user::user_route_config) // USER ROUTE
                .configure(routes::config::config_route_config),
        )
        // AUTH
        .wrap(HttpAuthentication::bearer(auth::bearer_auth_validator))
    );
}
