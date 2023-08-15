mod controllers;

use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::app::auth::guards;

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(controllers::register))
            .route("/login", web::post().to(controllers::login))
            .service(
                web::scope("/token")
                    .wrap(session.clone())
                    .route("", web::post().to(controllers::token)),
            )
            .service(
                web::scope("/logout")
                    .wrap(session)
                    .wrap(jwt)
                    .route("", web::post().to(controllers::logout)),
            ),
    );
}
