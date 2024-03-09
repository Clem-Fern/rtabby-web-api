use crate::app_config::MappedAppConfig;
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::warn;

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let default = web::Data::new(MappedAppConfig::default());
    let users: &Vec<String> = &req
        .app_data::<web::Data<MappedAppConfig>>()
        .unwrap_or(&default)
        .users
        .clone()
        .into_keys()
        .collect();
    
    let token = String::from(credentials.token());
    
    if users.contains(&token) {
        return Ok(req);
    }

    #[cfg(feature = "third-party-login")]
    {
        use crate::login::models::User;
        use crate::storage::DbPool;
        use actix_web::error::ErrorInternalServerError;

        let pool = req.app_data::<web::Data<DbPool>>().unwrap().clone();
        let token = token.clone();

        let result = web::block(move || {
            let mut conn = pool.get()?;
            User::get_user_by_token(&mut conn, &token)
        })
        .await;

        match result {
            Ok(result) => match result {
                Ok(result) => {
                    if let Some(_user) = result {
                        return Ok(req);
                    }
                }
                Err(err) => return Err((ErrorInternalServerError(err), req)),
            },
            Err(err) => return Err((ErrorInternalServerError(err), req)),
        }
    }

    warn!(
        "Authentification failed for {:?}",
        req.connection_info().peer_addr()
    );
    Err((ErrorUnauthorized("Invalide authentication token !"), req))
}
