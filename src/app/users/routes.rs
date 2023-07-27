use actix_web::web::{self, ServiceConfig};

use super::controllers::create_user;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::post().to(create_user)));
}
