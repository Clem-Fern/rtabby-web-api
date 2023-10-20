use actix_web::{get, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::app_config::MappedAppConfig;

#[get("/user")]
async fn get_user(
    auth: BearerAuth,
    app_config: web::Data<MappedAppConfig>,
) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());

    match app_config.users.get(&token) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}
