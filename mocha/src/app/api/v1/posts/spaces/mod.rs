mod controllers;

use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::app::guards;

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/spaces")
            // .wrap(session)
            // .wrap(jwt)
            .route("", web::get().to(controllers::get_spaces))
            .route("", web::post().to(controllers::create_space))
            .route("/{space}", web::get().to(controllers::get_space))
            .route("/{space}", web::put().to(controllers::edit_space))
            .route("/{space}", web::delete().to(controllers::delete_space))
            .route("/{space}/tags", web::get().to(controllers::get_tags))
            .route("/{space}/tags", web::post().to(controllers::create_tag))
            .service(
                web::scope("/tags")
                    .route("/{tag}", web::get().to(controllers::get_tag))
                    .route("/{tag}", web::put().to(controllers::edit_tag))
                    .route("/{tag}", web::delete().to(controllers::delete_tag)),
            ),
    );
}
