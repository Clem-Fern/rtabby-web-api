use std::collections::HashMap;

use actix_web::{get, web, Error, HttpResponse, HttpRequest};

use tera::Tera;
use serde::Deserialize;

use crate::login::github::Github;
use crate::login::gitlab::GitLab;
use crate::login::google::Google;
use crate::login::microsoft::Microsoft;
use crate::login::provider::ThirdPartyUserInfo;
use crate::login::provider::LoginProvider;
use crate::storage::DbPool;
use log::{info, error};

use crate::models::user::{User, NewUser};

use crate::env;

use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
    state: String,
}

#[get("/")]
async fn home(
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    if let Some(token) = req.cookie("token") {
        let mut context = tera::Context::new();
        context.insert("token", &token.value());
        let body = Tera::new(&(env::static_files_base_dir() + "templates/**/*")).unwrap().render("success.html", &context).unwrap();
        return Ok(HttpResponse::Ok().body(body));
    }
    let state = Uuid::new_v4().to_string();
    
    let mut platforms = Vec::<HashMap::<&str, String>>::new();

    let host = req.connection_info().host().to_string();
    platforms.push({
        let mut map = HashMap::new();
        map.insert("name", Github.name());
        map.insert("url", Github.login_url(host.clone(), state.clone()));
        map
    });

    platforms.push({
        let mut map = HashMap::new();
        map.insert("name", GitLab.name());
        map.insert("url", GitLab.login_url(host.clone(), state.clone()));
        map
    });

    platforms.push({
        let mut map = HashMap::new();
        map.insert("name", Google.name());
        map.insert("url", Google.login_url(host.clone(), state.clone()));
        map
    });

    platforms.push({
        let mut map = HashMap::new();
        map.insert("name", Microsoft.name());
        map.insert("url", Microsoft.login_url(host, state.clone()));
        map
    });

    let mut context = tera::Context::new();
    context.insert("platforms", &platforms);
    let body = Tera::new(&(env::static_files_base_dir() + "templates/**/*")).unwrap().render("login.html", &context).unwrap();

    let mut resp = HttpResponse::Ok()
    .body(body);
    let ret = resp.add_cookie(&actix_web::cookie::Cookie::build("state", &state)
    .path("/")
    .http_only(true)
    .expires(actix_web::cookie::time::OffsetDateTime::now_utc() + actix_web::cookie::time::Duration::minutes(5))
    .finish());
    if let Err(err) = ret {
        error!("add cookie failed: {}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(resp)
}

#[get("/login/microsoft/callback")]
async fn login_microsoft_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let host = req.connection_info().host().to_string();
    let user_info = Microsoft.user_info(host, info.code.clone()).await;
    login_callback(info, pool, req, user_info).await
}

#[get("/login/google/callback")]
async fn login_google_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let host = req.connection_info().host().to_string();
    let user_info = Google.user_info(host, info.code.clone()).await;
    login_callback(info, pool, req, user_info).await
}

#[get("/login/github/callback")]
async fn login_github_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let host = req.connection_info().host().to_string();
    let user_info = Github.user_info(host, info.code.clone()).await;
    login_callback(info, pool, req, user_info).await
}

#[get("/login/gitlab/callback")]
async fn login_gitlab_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let host = req.connection_info().host().to_string();
    let user_info = GitLab.user_info(host, info.code.clone()).await;
    login_callback(info, pool, req, user_info).await
}

async fn login_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
    user_info: Result<ThirdPartyUserInfo, Error>
) -> Result<HttpResponse, Error> {
    if let Some(state) =  req.cookie("state") {
        if state.value() != info.state {
            error!("state not match");
            let rediret = HttpResponse::Found()
            .append_header(("Location","/"))
            .finish();
            return Ok(rediret);
        }
    }
    else {
        error!("state not found");
        let rediret = HttpResponse::Found()
        .append_header(("Location","/"))
        .finish();
        return Ok(rediret);
    }

    if let Ok(user) = user_info {
        info!("user id: {}", user.id);
        let mut context = tera::Context::new();

        let clone_pool = pool.clone();
        let mid = user.id.clone();
        let mplatform = user.platform.clone();
        let current_user = web::block(move || {
            let mut conn = clone_pool.get()?;
            User::get_user(&mut conn, &mid, &mplatform)
        }).await.map_err(actix_web::error::ErrorInternalServerError)?;

        let current_user_token: String;
        if let Ok(Some(current_user)) = current_user {
            current_user_token = current_user.token;
            context.insert("token", &current_user_token);
            
        }
        else {
            let new_uuid = Uuid::new_v4().to_string();
            let new_user = NewUser {
                name: user.name,
                user_id: user.id,
                platform: user.platform.to_lowercase(),
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
        .append_header(("Location","/"))
        .cookie(actix_web::cookie::Cookie::build("token", &current_user_token)
        .path("/")
        .http_only(true)
        .finish())
        .finish();
        Ok(redirect)
    }
    else {
        error!("get user id failed");
        let rediret = HttpResponse::Found()
        .append_header(("Location","/"))
        .finish();
        Ok(rediret)
    }
}

pub fn user_login_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_github_callback);
    cfg.service(login_gitlab_callback);
    cfg.service(login_google_callback);
    cfg.service(login_microsoft_callback);
}