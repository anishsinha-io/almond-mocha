use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::tokens::{self, Claims};

// pub async fn session_guard(req: ServiceRequest) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)>{
//     if let Some(session_state)= req.cookie("session_state") {
//
//     }
// }

pub async fn jwt_guard(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    let result = tokens::verify_rs256(credentials.token());
    match result {
        Ok(jwt) => {
            req.extensions_mut().insert::<Claims>(jwt.claims);
            Ok(req)
        }
        Err(_) => Err((ErrorUnauthorized("invalid token".to_owned()), req)),
    }
}
