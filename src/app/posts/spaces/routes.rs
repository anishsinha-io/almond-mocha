use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::{create_space, delete_space, edit_space, get_space, get_spaces};
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
            .route("/{space}", web::delete().to(delete_space)),
    );
}
