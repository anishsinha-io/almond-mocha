mod controllers;

use crate::app::auth::guards;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/users")
            .wrap(session)
            .wrap(jwt)
            .route("", web::get().to(controllers::get_current_user))
            .route("", web::put().to(controllers::edit_current_user)),
    );
}
