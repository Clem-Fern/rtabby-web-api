use crate::login::env;
use crate::login::routes;
use actix_files as fs;
use actix_web::web;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::generate(),
            ))
            .service(routes::home)
            .configure(routes::user_login_route_config),
    )
    .configure(static_files_config);
}

pub fn static_files_config(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new(
        "/static",
        env::static_files_base_dir() + "static",
    ));
}
