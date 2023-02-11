use actix_web::{web, dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::{bearer::{BearerAuth}};
use log::warn;
use crate::app_config::MappedAppConfig;

pub async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let default = web::Data::new(MappedAppConfig::default());
    let users: &Vec<String> = &req.app_data::<web::Data<MappedAppConfig>>().unwrap_or(&default).users.clone().into_keys().collect();
    let token = credentials.token();
    if users.contains(&String::from(token)) {
        Ok(req)
    } else {
        warn!("Authentification failed for {:?}", req.connection_info().peer_addr());
        Err((ErrorUnauthorized("Invalide authentication token !"), req))
    }    
}