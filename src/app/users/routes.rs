use crate::app::auth::guards;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::create_user;

pub fn config(cfg: &mut ServiceConfig) {
    // cfg.service(web::scope("/users").route("", web::post().to(create_user)));

    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/users")
            .wrap(session)
            .wrap(jwt)
            .route("", web::post().to(create_user)),
    );
}
