use actix_web::{web, HttpResponse, Responder};

use super::dto;

pub async fn create_user(data: web::Json<dto::User>) -> impl Responder {
    HttpResponse::Ok().json(&data)
}
