
use actix_files as fs;
use actix_web::web;
use crate::login::env;
use crate::login::routes;

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/"))
    .configure(routes::user_login_route_config)
    ;
}

pub fn static_files_config(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/static", env::static_files_base_dir() + "static"));
}