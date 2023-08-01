use actix_web::{
    cookie::Cookie,
    web::{Data, Json},
    HttpResponse,
};
use uuid::Uuid;

use crate::app::{
    datasources::{postgres, users},
    dto::{CreateSession, CreateUser, LoginUser, RegisterUser},
    errors::AppError,
    launch::LaunchMode,
    state::AppState,
};

pub async fn register(
    state: Data<AppState>,
    data: Json<RegisterUser>,
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

    let new_user_id = users::create_user(&state.pool, dto)
        .await
        .map_err(|_| AppError::InternalServer)?;

    let new_session_state = format!("A+J_{}", Uuid::new_v4());

    let dto = CreateSession {
        user_id: new_user_id,
        session_state: new_session_state,
    };

    let session_state = postgres::auth::start_session(&state.pool, dto)
        .await
        .map_err(|_| AppError::InternalServer)?;

    let mut res =
        HttpResponse::Ok().json(serde_json::json!({"msg": "successfully created new user"}));

    res.add_cookie(&Cookie::new("session_state", session_state))
        .map_err(|_| AppError::InternalServer)?;
    Ok(res)
}

pub async fn login(state: Data<AppState>, data: Json<LoginUser>) {
    let raw_data = data.into_inner();
}

pub async fn logout() {}
