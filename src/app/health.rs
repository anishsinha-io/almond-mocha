use actix_web::{web::Json, Responder, Result};

pub async fn health() -> Result<impl Responder> {
    Ok(Json(serde_json::json!(r#"{"msg": "app is healthy"}"#)))
}
