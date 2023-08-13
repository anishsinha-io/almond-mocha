use actix_web::web::{self, ServiceConfig};

use super::spaces::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/posts").configure(routes::config));
}
