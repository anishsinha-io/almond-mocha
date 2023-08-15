use actix_web::{
    web::{Data, Json, ReqData},
    HttpResponse,
};

use crate::app::{
    auth::tokens::Claims,
    dto::users::{EditUser, EditUserInfo, GetUserById},
    errors::AppError,
    state::AppState,
    storage::postgres,
};

pub async fn get_current_user(
    state: Data<AppState>,
    claims: ReqData<Claims>,
) -> actix_web::Result<HttpResponse, AppError> {
    let user_id = claims.sub.clone();

    let dto = GetUserById { id: user_id };
    let user = postgres::users::get_user_by_id(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match user {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => {
            log::error!("this branch should not be reachable. something is critically wrong.");
            Err(AppError::NotFound)?
        }
    }
}

pub async fn edit_current_user(
    state: Data<AppState>,
    claims: ReqData<Claims>,
    data: Json<EditUserInfo>,
) -> actix_web::Result<HttpResponse, AppError> {
    let id = claims.sub.to_string();
    let maybe_user = postgres::users::get_user_by_id(&state.storage_layer.pg, GetUserById { id })
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match maybe_user {
        Some(user) => {
            let edit_info = data.into_inner();
            let dto = EditUser {
                id: claims.sub.to_string(),
                first_name: edit_info.first_name.unwrap_or(user.first_name),
                last_name: edit_info.last_name.unwrap_or(user.last_name),
                username: edit_info.username.unwrap_or(user.username),
                image_uri: edit_info.image_uri.unwrap_or(user.image_uri),
            };
            match postgres::users::edit_user(&state.storage_layer.pg, dto).await {
                Ok(_) => Ok(HttpResponse::Created()
                    .json(serde_json::json!({"msg": "successfully edited user"}))),
                Err(_) => Err(AppError::InternalServerError)?,
            }
        }
        None => {
            log::error!("this branch should not be reachable. something is critically wrong.");
            Err(AppError::NotFound)?
        }
    }
}
