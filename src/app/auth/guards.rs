use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, web::Data};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::app::entities::auth::Session;
use crate::app::state::AppState;

use super::tokens::{self, Claims};

pub async fn session_guard(
    req: ServiceRequest,
    state: Data<AppState>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let cookie_value = req
        .cookie("mocha_session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or("".to_owned());

    let result = state
        .session_manager
        .check_session(&state.storage_layer, &cookie_value)
        .await;

    match result {
        Ok(session) => {
            req.extensions_mut().insert::<Session>(session);
            Ok(req)
        }
        Err(_) => Err((
            ErrorUnauthorized(serde_json::json!({"error": "invalid session"})),
            req,
        )),
    }
}

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
