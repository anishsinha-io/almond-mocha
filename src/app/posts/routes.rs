use actix_web::web::{self, ServiceConfig};

use super::{controllers::create_sticker, spaces};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .configure(spaces::routes::config)
            .service(web::scope("/stickers").route("", web::post().to(create_sticker))),
    );
}
