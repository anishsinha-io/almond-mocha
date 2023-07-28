use actix_web::{HttpResponse, Responder};

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"msg": "here"}))
}
