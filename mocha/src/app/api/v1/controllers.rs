use actix_web::{http::header::ContentType, HttpResponse, Responder};

pub async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(serde_json::json!({"msg": "app is healthy"}))
}
