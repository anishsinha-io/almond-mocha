use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::{
    create_space, create_tag, delete_space, delete_tag, edit_space, edit_tag, get_space,
    get_spaces, get_tag, get_tags,
};
use crate::app::guards;

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/spaces")
            // .wrap(session)
            // .wrap(jwt)
            .route("", web::get().to(get_spaces))
            .route("", web::post().to(create_space))
            .route("/{space}", web::get().to(get_space))
            .route("/{space}", web::put().to(edit_space))
            .route("/{space}", web::delete().to(delete_space))
            .route("/{space}/tags", web::get().to(get_tags))
            .route("/{space}/tags", web::post().to(create_tag))
            .service(
                web::scope("/tags")
                    .route("/{tag}", web::get().to(get_tag))
                    .route("/{tag}", web::put().to(edit_tag))
                    .route("/{tag}", web::delete().to(delete_tag)),
            ),
    );
}
