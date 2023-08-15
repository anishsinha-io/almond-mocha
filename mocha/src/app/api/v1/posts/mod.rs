mod controllers;
mod spaces;

use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/posts").configure(spaces::config).service(
            web::scope("/stickers").route("", web::post().to(controllers::create_sticker)),
        ),
    );
}
