use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    web::{Data, Json, ReqData},
    HttpRequest, HttpResponse,
};

use crate::app::{
    dto::{CreateSession, CreateUser, DeleteSession, GetUserByEmail, LoginUser, RegisterUser},
    errors::AppError,
    launch::LaunchMode,
    state::AppState,
    storage::{entities::Session, postgres, users},
};

use super::tokens::Claims;

pub async fn register(
    state: Data<AppState>,
    data: Json<RegisterUser>,
) -> actix_web::Result<HttpResponse, AppError> {
    // Do NOT allow registration in production.
    if state.config.launch_mode == LaunchMode::Production {
        log::warn!("registration is not allowed in production or staging environments. if you are in development, please set the LAUNCH_MODE environment variable to 'development' or 'testing'");
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

    let new_user_id = users::create_user(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let session_id = state
        .session_manager
        .start_session(
            &state.storage_layer,
            CreateSession {
                user_id: new_user_id.clone(),
                data: serde_json::json!({}),
                created_at: chrono::offset::Utc::now(),
                updated_at: chrono::offset::Utc::now(),
            },
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let session_cookie = state
        .session_manager
        .create_signed_cookie(&session_id)
        .map_err(|_| AppError::InternalServerError)?;

    let access_token = Claims::default(&new_user_id.clone())
        .sign_rs256()
        .map_err(|_| AppError::InternalServerError)?;

    let mut res = HttpResponse::Ok().json(
        serde_json::json!({"msg": "successfully created new user", "access_token": access_token}),
    );

    let mut cookie = Cookie::new("mocha_session", &session_cookie);
    cookie.set_http_only(true);

    let mut cookie_expiration = OffsetDateTime::now_utc();
    cookie_expiration += Duration::weeks(52);
    cookie.set_expires(cookie_expiration);
    cookie.set_path("/");

    match state.config.launch_mode {
        LaunchMode::Production | LaunchMode::Staging => cookie.set_secure(true),
        _ => (),
    };

    res.add_cookie(&cookie)
        .map_err(|_| AppError::InternalServerError)?;
    Ok(res)
}

pub async fn token(
    state: Data<AppState>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse, AppError> {
    let cookie = req.cookie("mocha_session");
    match cookie {
        Some(cookie_data) => {
            // let value = cookie_data.value();
            // log::debug!("{value}");
            let session = state
                .session_manager
                .check_session(&state.storage_layer, cookie_data.value())
                .await
                .map_err(|_| AppError::Unauthorized)?;

            let access_token = Claims::default(&session.user_id.to_string())
                .sign_rs256()
                .map_err(|_| AppError::InternalServerError)?;

            Ok(HttpResponse::Ok().json(serde_json::json!({ "access_token": access_token })))
        }
        None => Err(AppError::Unauthorized),
    }
}

pub async fn login(
    state: Data<AppState>,
    data: Json<LoginUser>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse, AppError> {
    let existing_session_cookie = req.cookie("mocha_session");
    if let Some(session_cookie) = existing_session_cookie {
        let session_data = state
            .session_manager
            .verify_session_signature(session_cookie.value())
            .map_err(|_| AppError::Forbidden)?;

        let id = session_data["session_id"].as_str().unwrap_or("");

        state
            .session_manager
            .end_session(&state.storage_layer, DeleteSession { id: id.to_owned() })
            .await
            .map_err(|_| AppError::InternalServerError)?
    };

    let raw_data = data.into_inner();

    let dto = GetUserByEmail {
        email: raw_data.email,
    };

    // match
    match postgres::users::get_user_with_credentials_by_email(&state.storage_layer.pg, dto)
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

                let session_id = state
                    .session_manager
                    .start_session(
                        &state.storage_layer,
                        CreateSession {
                            user_id: user.id.clone().to_string(),
                            data: serde_json::json!({}),
                            created_at: chrono::offset::Utc::now(),
                            updated_at: chrono::offset::Utc::now(),
                        },
                    )
                    .await
                    .map_err(|_| AppError::InternalServerError)?;

                let session_cookie = state
                    .session_manager
                    .create_signed_cookie(&session_id)
                    .map_err(|_| AppError::InternalServerError)?;

                let mut res =
                    HttpResponse::Ok().json(serde_json::json!({ "access_token": access_token }));

                let mut cookie = Cookie::new("mocha_session", &session_cookie);
                cookie.set_http_only(true);

                let mut cookie_expiration = OffsetDateTime::now_utc();
                cookie_expiration += Duration::weeks(52);
                cookie.set_expires(cookie_expiration);
                cookie.set_path("/");

                match state.config.launch_mode {
                    LaunchMode::Production | LaunchMode::Staging => cookie.set_secure(true),
                    _ => (),
                };

                res.add_cookie(&cookie)
                    .map_err(|_| AppError::InternalServerError)?;
                Ok(res)
            } else {
                Err(AppError::Unauthorized)
            }
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn logout(
    state: Data<AppState>,
    session: ReqData<Session>,
) -> actix_web::Result<HttpResponse, AppError> {
    let dto = DeleteSession {
        id: session.id.clone().to_string(),
    };
    state
        .session_manager
        .end_session(&state.storage_layer, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let mut res = HttpResponse::Ok().json(serde_json::json!({"msg": "successfully logged out"}));

    res.del_cookie("mocha_session");
    Ok(res)
}
