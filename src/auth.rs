use actix_web::{web, dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::warn;
use crate::app_config::MappedAppConfig;
#[cfg(feature = "third-party-login")]
use crate::storage::DbPool;
#[cfg(feature = "third-party-login")]
use crate::login::models::User;

pub async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let default = web::Data::new(MappedAppConfig::default());
    let users: &Vec<String> = &req.app_data::<web::Data<MappedAppConfig>>().unwrap_or(&default).users.clone().into_keys().collect();
    let token = String::from(credentials.token()); 
    #[cfg(feature = "third-party-login")]
    let mtoken = token.clone();
    if users.contains(&token) {
        Ok(req)
    } else {
        #[cfg(feature = "third-party-login")]
        {
            let pool = req.app_data::<web::Data<DbPool>>().unwrap().clone(); 
            if let Some(_user) = web::block(move || { 
                let mut conn = pool.get()?; 
                User::get_user_by_token(&mut conn, &mtoken) 
            })     
            .await.unwrap().unwrap() { 
                Ok(req) 
            } else { 
                warn!("Authentification failed for {:?}", req.connection_info().peer_addr()); 
                Err((ErrorUnauthorized("Invalide authentication token !"), req)) 
            } 
        }
        
        #[cfg(not(feature = "third-party-login"))]
        {
            warn!("Authentification failed for {:?}", req.connection_info().peer_addr());
            Err((ErrorUnauthorized("Invalide authentication token !"), req))
        }
    }    
}