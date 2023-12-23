use actix_web::{get, web, Error, HttpResponse, HttpRequest};

use tera::Tera;
use serde::Deserialize;
use std::collections::HashMap;

use crate::storage::DbPool;

use log::{info, error};

use crate::models::user::{User, NewUser};

use crate::env;

use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    access_token: String,
}


#[derive(Debug, Deserialize)]
pub struct UserInfo {
    id: i32,
    name: String,
}

#[get("/login")]
async fn login(
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // get code parameter from request
    let mut context = tera::Context::new();
    println!("client_id: {}", env::ENV_GITHUB_APP_CLIENT_ID);
    let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
    let login_url = format!( "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}://{}/login/github/callback", client_id, req.connection_info().scheme(), req.connection_info().host());
    context.insert("login_url", &login_url);
    let body = Tera::new("src/templates/**/*").unwrap().render("home.tpl", &context).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

async fn get_user_info(
    token: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.github.com/user")
    .header("Authorization", format!("Bearer {}", token))
    .header("User-Agent", "actix-web/3.3.2")
    .header("X-GitHub-Api-Version", "2022-11-28")
    .header("Accept", "application/vnd.github.v3+json")
    .send()
    .await;
    return res;
}


#[get("/login/success")]
async fn login_success(
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // get token from cookie
    if let Some(token) = req.cookie("token") {
        let mut context = tera::Context::new();
        context.insert("token", &token.value());
        let body = Tera::new("src/templates/**/*").unwrap().render("success.tpl", &context).unwrap();
        return Ok(HttpResponse::Ok().body(body));
    }
    else {
        let context = tera::Context::new();
        let body = Tera::new("src/templates/**/*").unwrap().render("error.tpl", &context).unwrap();
        return Ok(HttpResponse::Ok().body(body));
    }
}

#[get("/login/github/callback")]
async fn login_github_callback(
    info: web::Query<Params>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("code", &info.code);
    let client_id = env::var(env::ENV_GITHUB_APP_CLIENT_ID).expect("Missing GITHUB_APP_CLIENT_ID env var");
    let client_secret = env::var(env::ENV_GITHUB_APP_CLIENT_SECRET).expect("Missing GITHUB_APP_CLIENT_SECRET env var");
    map.insert("client_id", &client_id);
    map.insert("client_secret", &client_secret);

    let res = client.post("https://github.com/login/oauth/access_token")
    .json(&map)
    .header("Accept", "application/json")
    .send()
    .await;
    // print res body
    if let Ok(res) = res {
        let body = res.json::<Body>().await;
        if let Ok(body) = body {
            if let Ok(user_info_resp) = get_user_info(body.access_token).await {
                let user_info = user_info_resp.json::<UserInfo>().await;
                if let Ok(user_info) = user_info {
                    info!("login success");

                    let mut context = tera::Context::new();

                    let clone_pool = pool.clone();
                    let current_user = web::block(move || {
                        let mut conn = clone_pool.get()?;
                        return User::get_user(&mut conn, &user_info.id.to_string(), "github");
                    }).await.map_err(actix_web::error::ErrorInternalServerError)?;

                    let current_user_token: String;
                    if let Ok(Some(current_user)) = current_user {
                        current_user_token = current_user.token.clone();
                        context.insert("token", &current_user_token);
                        
                    }
                    else {
                        let new_uuid = Uuid::new_v4().to_string();
                        let new_user = NewUser {
                            name: user_info.name.clone(),
                            user_id: user_info.id.to_string(),
                            platform: String::from("github"),
                            token: new_uuid.clone(),
                        };
                        web::block(move || {
                            let mut conn = pool.get()?;
                            return User::insert_new_user_config(&mut conn, new_user);
                        })
                        .await?
                        .map_err(actix_web::error::ErrorInternalServerError)?;

                        context.insert("token", &new_uuid);
                        current_user_token = new_uuid;
                    }

                    // redirect to login success page with 302, and set cookie
                    let redirect = HttpResponse::Found()
                    .append_header(("Location", "/login/success"))
                    .cookie(actix_web::cookie::Cookie::build("token", &current_user_token)
                    .path("/")
                    .http_only(true)
                    .finish())
                    .finish();
                    return Ok(redirect);
                }
                else {
                    error!("login failed");
                    let context = tera::Context::new();
                    let body = Tera::new("src/templates/**/*").unwrap().render("error.tpl", &context).unwrap();
                    return Ok(HttpResponse::Ok().body(body));
                }
            }
        }
    }
    let context = tera::Context::new();
    let body = Tera::new("src/templates/**/*").unwrap().render("home.tpl", &context).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

pub fn user_login_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(login_github_callback);
    cfg.service(login_success);
}