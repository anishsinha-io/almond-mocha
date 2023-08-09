use actix_web::{
    cookie::Cookie,
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::app::{
    datasources::{postgres, users},
    dto::{CreateSession, CreateUser, GetUserByEmail, LoginUser, RegisterUser},
    errors::AppError,
    launch::LaunchMode,
    state::AppState,
};

use super::tokens::Claims;

pub async fn register(
    state: Data<AppState>,
    data: Json<RegisterUser>,
) -> actix_web::Result<HttpResponse, AppError> {
    // Do NOT allow registration in production.
    if state.launch_mode == LaunchMode::Production {
        return Err(AppError::Forbidden);
    };

    let raw_data = data.into_inner();

    let alg = state.credential_manager.algorithm.clone();

    let mut dto = CreateUser {
        first_name: raw_data.first_name,
        last_name: raw_data.last_name,
        email: raw_data.email,
        username: raw_data.username,
        image_uri: raw_data.image_uri,
        hashed_password: None,
        algorithm: None,
    };

    if let Some(plaintext) = raw_data.password {
        let hashed_password = state
            .credential_manager
            .create_hash(plaintext.as_bytes())
            .map_err(|_| AppError::InternalServerError)?;
        dto.hashed_password = Some(hashed_password);
        dto.algorithm = Some(alg);
    }

    let new_user_id = users::create_user(&state.pool, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let new_session_state = format!("A+J_{}", Uuid::new_v4());

    // let dto = CreateSession {
    //     user_id: new_user_id.clone(),
    //     session_state: new_session_state,
    // };

    // let session_state = postgres::auth::start_session(&state.pool, dto)
    //     .await
    //     .map_err(|_| AppError::InternalServerError)?;

    // let access_token = Claims::default(&new_user_id.clone())
    //     .sign_rs256()
    //     .map_err(|_| AppError::InternalServerError)?;
    //
    // let mut res = HttpResponse::Ok().json(
    //     serde_json::json!({"msg": "successfully created new user", "access_token": access_token}),
    // );
    //
    // res.add_cookie(&Cookie::new("session_state", session_state))
    //     .map_err(|_| AppError::InternalServerError)?;
    // Ok(res)
    Err(AppError::BadRequest)
}

pub async fn token(state: Data<AppState>, req: HttpRequest) {}

pub async fn login(
    state: Data<AppState>,
    data: Json<LoginUser>,
) -> actix_web::Result<HttpResponse, AppError> {
    let raw_data = data.into_inner();

    let dto = GetUserByEmail {
        email: raw_data.email,
    };

    // match
    match postgres::users::get_user_with_credentials_by_email(&state.pool, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?
    {
        Some(user) => {
            let candidate = raw_data.password;
            let hash = user.credential_hash;
            if state.credential_manager.verify_hash(&candidate, &hash) {
                let access_token = Claims::default(&user.id.to_string())
                    .sign_rs256()
                    .map_err(|_| AppError::InternalServerError)?;

                // let new_session_state = format!("A+J_{}", Uuid::new_v4());

                // let dto = CreateSession {
                //     user_id: user.id.to_string(),
                //     session_state: new_session_state,
                // };
                //
                // let session_state = postgres::auth::start_session(&state.pool, dto)
                //     .await
                //     .map_err(|_| AppError::InternalServerError)?;
                //
                // let mut res = HttpResponse::Ok().json(
                //     serde_json::json!({"msg":"successfully logged in", "access_token": access_token}),
                // );
                // res.add_cookie(&Cookie::new("session_state", session_state))
                //     .map_err(|_| AppError::InternalServerError)?;

                Err(AppError::BadRequest)
            } else {
                Err(AppError::Unauthorized)
            }
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn logout(req: HttpRequest, state: Data<AppState>) {
    let claims = req.extensions().get::<Claims>();
}
