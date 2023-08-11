use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::{login, logout, register, token};

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(super::guards::session_guard);
    let jwt = HttpAuthentication::bearer(super::guards::jwt_guard);
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .service(
                web::scope("/token")
                    .wrap(session.clone())
                    .route("", web::post().to(token)),
            )
            .service(
                web::scope("/logout")
                    .wrap(session)
                    .wrap(jwt)
                    .route("", web::post().to(logout)),
            ),
    );
}
