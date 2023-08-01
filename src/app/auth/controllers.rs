use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::app::{
    datasources::users,
    dto::{CreateUser, NewUserData},
    errors::AppError,
    launch::LaunchMode,
    state::AppState,
};

pub async fn register(
    state: Data<AppState>,
    data: Json<NewUserData>,
) -> actix_web::Result<HttpResponse, AppError> {
    // Do NOT allow registration in production.
    if state.launch_mode == LaunchMode::Production {
        return Err(AppError::Forbidden);
    };

    let raw_data = data.into_inner();

    let alg = state.manager.algorithm.clone();

    let mut dto = CreateUser {
        first_name: raw_data.first_name,
        last_name: raw_data.last_name,
        email: raw_data.email,
        username: raw_data.username,
        image_uri: raw_data.image_uri,
        hashed_password: None,
        algorithm: None,
    };

    if let Some(plaintext) = raw_data.plaintext_password {
        let hashed_password = state
            .manager
            .create_hash(plaintext.as_bytes())
            .map_err(|_| AppError::InternalServer)?;
        dto.hashed_password = Some(hashed_password);
        dto.algorithm = Some(alg);
    }

    users::create_user(&state.pool, dto)
        .await
        .map_err(|_| AppError::InternalServer)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "successfully created new user"})))
}

pub async fn login() {}

pub async fn logout() {}
