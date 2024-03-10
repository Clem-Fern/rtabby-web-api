use actix_web::http::header::ContentType;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};

use serde::Deserialize;
use tera::Tera;

use crate::login::error::ProviderError;
use crate::login::providers::Platform;
use crate::login::ProvidersConfig;
use crate::storage::DbPool;
use log::{error, info};

use crate::login::models::{NewUser, User};

use crate::env as app_;
use crate::login::env;

use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
    state: String,
}

#[get("")]
async fn home(
    req: HttpRequest,
    providers_config: web::Data<ProvidersConfig>,
) -> Result<HttpResponse, Error> {
    if let Some(token) = req.cookie("token") {
        let mut context = tera::Context::new();
        context.insert("token", &token.value());
        let version = env!("CARGO_PKG_VERSION");
        if let Ok(hash) = app_::var("GIT_COMMIT") {
            context.insert("version", &format!("{} ({})", version, hash));
        } else {
            context.insert("version", &version);
        }
        let body = Tera::new(&(env::static_files_base_dir() + "templates/**/*"))
            .map_err(actix_web::error::ErrorInternalServerError)?
            .render("success.html", &context)
            .map_err(actix_web::error::ErrorInternalServerError)?;
        return Ok(HttpResponse::build(actix_web::http::StatusCode::OK)
            .content_type(ContentType::html())
            .body(body));
    }

    let platforms: Vec<Platform> = providers_config
        .available_providers
        .clone()
        .into_iter()
        .map(|p| p.into())
        .collect();

    let mut context = tera::Context::new();
    context.insert("platforms", &platforms);
    let version = env!("CARGO_PKG_VERSION");
    if let Ok(hash) = app_::var("GIT_COMMIT") {
        context.insert("version", &format!("{} ({})", version, hash));
    } else {
        context.insert("version", &version);
    }
    let body = Tera::new(&(env::static_files_base_dir() + "templates/**/*"))
        .map_err(actix_web::error::ErrorInternalServerError)?
        .render("login.html", &context)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[get("/{login}")]
async fn login(
    provider_name: web::Path<String>,
    providers_config: web::Data<ProvidersConfig>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let provider_name = provider_name.into_inner();
    let provider = providers_config
        .available_providers
        .clone()
        .into_iter()
        .find(|p| p.name().eq(&provider_name))
        .ok_or(ProviderError::NotFound(provider_name))
        .map_err(actix_web::error::ErrorBadRequest)?;

    let host = req.connection_info().host().to_string();
    let state = Uuid::new_v4().to_string();

    let login_url = provider.get_login_url(host, state.clone());

    let mut response = HttpResponse::TemporaryRedirect()
        .append_header(("Location", login_url))
        .finish();

    let ret = response.add_cookie(
        &actix_web::cookie::Cookie::build("state", &state)
            .path("/")
            .expires(
                actix_web::cookie::time::OffsetDateTime::now_utc()
                    + actix_web::cookie::time::Duration::minutes(5),
            )
            .finish(),
    );

    if let Err(err) = ret {
        error!("add cookie failed: {}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(response)
}

#[get("/{login}/callback")]
async fn login_callback(
    provider_name: web::Path<String>,
    providers_config: web::Data<ProvidersConfig>,
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let provider_name = provider_name.into_inner();
    let provider = providers_config
        .available_providers
        .clone()
        .into_iter()
        .find(|p| p.name().eq(&provider_name))
        .ok_or(ProviderError::NotFound(provider_name))
        .map_err(actix_web::error::ErrorBadRequest)?;

    if let Some(state) = req.cookie("state") {
        if state.value() != info.state {
            error!("state not match");
            let rediret = HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish();
            return Ok(rediret);
        }
    } else {
        error!("state not found");
        let rediret = HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish();
        return Ok(rediret);
    }

    let host = req.connection_info().host().to_string();

    let user_info = provider
        .get_user_info(host, info.code.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    info!("user id: {}", user_info.id);
    let mut context = tera::Context::new();

    let clone_pool = pool.clone();
    let mid = user_info.id.clone();
    let mplatform = user_info.platform.clone();
    let current_user = web::block(move || {
        let mut conn = clone_pool.get()?;
        User::get_user(&mut conn, &mid, &mplatform)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let current_user_token: String;
    if let Ok(Some(current_user)) = current_user {
        current_user_token = current_user.token;
        context.insert("token", &current_user_token);
    } else {
        let new_uuid = Uuid::new_v4().to_string();
        let new_user = NewUser {
            name: user_info.name,
            user_id: user_info.id,
            platform: user_info.platform,
            token: new_uuid.clone(),
        };
        web::block(move || {
            let mut conn = pool.get()?;
            User::insert_new_user_config(&mut conn, new_user)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        context.insert("token", &new_uuid);
        current_user_token = new_uuid;
    }

    // redirect to login success page with 302, and set cookie
    let redirect = HttpResponse::Found()
        .append_header(("Location", "/"))
        .cookie(
            actix_web::cookie::Cookie::build("token", &current_user_token)
                .path("/")
                .finish(),
        )
        .finish();
    Ok(redirect)
}

pub fn user_login_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(login_callback);
}
