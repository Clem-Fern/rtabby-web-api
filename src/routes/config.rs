use actix_web::{delete, get, patch, post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::storage::DbPool;

use crate::models::config::{
    Config, ConfigWithoutUser, ConfigWithoutUserAndContent, NewConfig, UpdateConfig,
};

#[get("/configs")]
async fn show_configs(auth: BearerAuth, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());

    let mtoken = token.clone();
    let mpool = pool.clone();
    let configs = web::block(move || {
        let mut conn = mpool.get()?;
        Config::get_all_config_by_user(&mut conn, &mtoken)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(configs))
}

#[post("/configs")] // create a new config
async fn new_config(
    auth: BearerAuth,
    pool: web::Data<DbPool>,
    json: web::Json<NewConfig>,
) -> Result<HttpResponse, Error> {
    let token = auth.token();
    let new_user_config = json
        .into_inner()
        .into_new_user_config_with_user(String::from(token));

    web::block(move || {
        let mut conn = pool.get()?;
        Config::insert_new_user_config(&mut conn, new_user_config)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/configs/{id}")]
async fn get_config(
    auth: BearerAuth,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get()?;
        Config::get_config_by_id_and_user(&mut conn, id, &token)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Some(config) => Ok(HttpResponse::Ok().json(Into::<ConfigWithoutUser>::into(config))),
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[patch("/configs/{id}")]
async fn update_config(
    auth: BearerAuth,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<UpdateConfig>,
) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());
    let id = path.into_inner();
    let updated_config = json.into_inner();

    let t = token.clone();
    let p = pool.clone();
    let config = web::block(move || {
        let mut conn = p.get()?;
        Config::get_config_by_id_and_user(&mut conn, id, &t)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if config.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let config = config.unwrap();
    let c = config.clone();
    web::block(move || {
        // update config content
        let mut conn = pool.get()?;
        Config::update_user_config_content(&mut conn, c, &updated_config.content)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(Into::<ConfigWithoutUserAndContent>::into(config.clone())))
}

#[delete("/configs/{id}")]
async fn delete_config(
    auth: BearerAuth,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let token = String::from(auth.token());
    let id = path.into_inner();

    let t = token.clone();
    let p = pool.clone();
    let config = web::block(move || {
        let mut conn = p.get()?;
        Config::get_config_by_id_and_user(&mut conn, id, &t)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if config.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let config = config.unwrap();
    web::block(move || {
        // delete config
        let mut conn = pool.get()?;
        Config::delete_config(&mut conn, config)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn config_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_configs)
        .service(new_config)
        .service(get_config)
        .service(update_config)
        .service(delete_config);
}
