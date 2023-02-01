extern crate env_logger;
extern crate log;
use log::{info, error};

use std::error::Error;

mod config;
mod env;
mod error;
use config::{Config, MappedConfig};
mod storage;
use storage::Storage;

mod auth;
mod models;
mod routes;
mod schema;

extern crate serde_yaml;

extern crate actix_web;
use actix_web::{middleware, web, App, HttpServer};

extern crate actix_web_httpauth;
use actix_web_httpauth::middleware::HttpAuthentication;

mod tls;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // INITIALIZE LOGGING
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        if cfg!(debug_assertions) {
            "trace"
        } else {
            "info"
        },
    ))
    /*.format(|buf, record| {
        use io::Write;
        let ts = buf.timestamp();
        writeln!(buf, "[{} {} {}] {}", ts, buf.default_styled_level(record.level()), env!("CARGO_PKG_NAME"), record.args())
    })*/
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
    let config: Config = config::load_file(&config_file_name)?;
    let config: MappedConfig = config.into();

    info!("{} loaded => {} users found, {} shared configs found", config_file_name, config.users.len(), config.shared_configs.len());

    // INIT DATABASE STORAGE
    let storage: Storage = Storage::new();
    let pool = storage.clone().pool()?;
    storage.init()?;

    // TODO : storage clean up on start

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone())) // App Config Data
            .app_data(web::Data::new(pool.clone())) // Database Pool Data
            .wrap(middleware::Logger::default().log_target(env!("CARGO_PKG_NAME").to_string()))
            // AUTH
            .wrap(HttpAuthentication::bearer(auth::bearer_auth_validator))
            //
            .configure(api_v1_config)
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
        server = server.bind_rustls(format!("{bind_addr}:{bind_port}"), config)?;
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
        ),
    );
}
