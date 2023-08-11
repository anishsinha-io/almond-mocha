use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::{login, register, token};

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(super::guards::session_guard);
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .wrap(session)
            .route("/token", web::post().to(token)),
    );
}
