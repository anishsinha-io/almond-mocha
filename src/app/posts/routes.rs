use actix_web::web::{self, ServiceConfig};

use super::spaces;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/posts").configure(spaces::routes::config));
}
