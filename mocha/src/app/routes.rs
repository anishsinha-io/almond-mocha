use crate::app::api;
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.configure(api::v1::config);
}
