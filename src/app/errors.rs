use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[allow(dead_code)]
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "bad request")]
    BadRequest,
    #[display(fmt = "unauthorized")]
    Unauthorized,
    #[display(fmt = "forbidden")]
    Forbidden,
    #[display(fmt = "internal server error")]
    InternalServer,
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(serde_json::json!({"error": self.to_string()}))
    }
}
