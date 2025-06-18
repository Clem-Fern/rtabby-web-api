use actix_web::{get, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::app_config::MappedAppConfig;

#[cfg(feature = "third-party-login")]
use crate::login::models::User;
use crate::storage::DbPool;

#[allow(unused_variables)]
#[get("/user")]
async fn get_user(
    auth: BearerAuth,
    app_config: web::Data<MappedAppConfig>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());
    #[cfg(feature = "third-party-login")]
    {
        let clone_pool = pool.clone();
        let clone_token = token.clone();
        let current_user = web::block(move || {
            let mut conn = clone_pool.get()?;
            User::get_user_by_token(&mut conn, &clone_token)
        })
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
        if let Ok(Some(current_user)) = current_user {
            return Ok(HttpResponse::Ok().json(current_user));
        }
    }

    match app_config.users.get(&token) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}
