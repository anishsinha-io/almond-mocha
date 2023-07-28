use actix_web::web::{self, ServiceConfig};

pub async fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/posts"));
}
