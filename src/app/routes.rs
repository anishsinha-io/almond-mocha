use actix_web::web;
use actix_web::web::ServiceConfig;

use super::controllers;
use super::users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(controllers::health)))
        .service(web::scope("/v1").configure(users::routes::config));
}
