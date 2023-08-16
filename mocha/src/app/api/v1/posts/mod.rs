mod controllers;
mod requests;
mod spaces;

use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::app::guards;

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/posts").configure(spaces::config).service(
            web::scope("/stickers")
                .wrap(session)
                .wrap(jwt)
                .route("", web::get().to(controllers::get_user_created_stickers))
                .route("", web::post().to(controllers::create_stickers))
                .route("/{sticker}", web::put().to(controllers::edit_sticker))
                .route("/{sticker}", web::delete().to(controllers::delete_sticker))
                .route(
                    "/available",
                    web::get().to(controllers::get_available_stickers),
                ),
        ),
    );
}
