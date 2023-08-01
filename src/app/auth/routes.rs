use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use super::controllers::register;

pub fn config(cfg: &mut ServiceConfig) {
    // cfg.service(web::resource("/auth").route(web::post().to(register)));
    cfg.service(web::scope("/auth").route("/register", web::post().to(register)));
}
