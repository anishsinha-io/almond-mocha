use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/auth").route(web::get().to(|| async { HttpResponse::Ok().body("hey") })),
    );
}
