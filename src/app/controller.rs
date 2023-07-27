use actix_web::web;
use actix_web::web::ServiceConfig;

use super::health;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health::health)))
        .service(web::scope("/v1"));
}
