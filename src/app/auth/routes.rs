use actix_web::web::{self, ServiceConfig};

use super::controllers::{login, register};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}
