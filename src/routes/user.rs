use actix_web::{web, HttpResponse, get, put};

#[get("/user")]
async fn get_user() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[put("/user")]
async fn update_user() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
    .service(update_user);
}
