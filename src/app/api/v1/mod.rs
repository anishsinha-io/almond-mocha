mod admin;
mod auth;
mod controllers;
mod posts;
mod users;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(controllers::health)))
        .service(
            web::scope("/v1")
                .configure(auth::config)
                .configure(users::config)
                .configure(posts::config),
        );
}
