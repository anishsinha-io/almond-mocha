use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};
use actix_web_grants::proc_macro::has_permissions;

use crate::app::{
    auth::tokens::Claims,
    dto::stickers::{
        CreateSticker, CreateStickers, DeleteSticker, EditSticker, GetAvailableStickers,
        GetStickersByUser,
    },
    errors::AppError,
    state::AppState,
    storage::postgres,
    types::AssetVisibility,
    upload,
};

use super::requests::EditStickerRequest;

#[has_permissions("stickers:create")]
pub async fn create_stickers(
    state: Data<AppState>,
    payload: Multipart,
    claims: ReqData<Claims>,
) -> actix_web::Result<HttpResponse, AppError> {
    let uploaded_assets = upload::files::save_assets(state.config.asset_backend, payload).await;
    let token_claims = claims.into_inner();
    let user_id = token_claims.sub;

    match uploaded_assets {
        Ok(uploads) => {
            let stickers: Vec<CreateSticker> = uploads
                .into_iter()
                .map(|u| CreateSticker {
                    backend: state.config.asset_backend,
                    file_path: u.file_path,
                    #[rustfmt::skip]
                    visibility: if u.friendly_name.ends_with(":private") { AssetVisibility::Private } else { AssetVisibility::Public },
                    friendly_name: u.friendly_name,
                })
                .collect();

            let dto = CreateStickers { user_id, stickers };

            let _ = postgres::stickers::create_stickers(&state.storage_layer.pg, dto)
                .await
                .map_err(|_| AppError::InternalServerError);

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "msg": format!("successfully created new sticker(s)",)
            })))
        }
        Err(e) => {
            log::error!("error creating sticker: {e}");
            Err(AppError::InternalServerError)
        }
    }
}

#[has_permissions("stickers:get")]
pub async fn get_user_created_stickers(
    state: Data<AppState>,
    claims: ReqData<Claims>,
) -> actix_web::Result<HttpResponse, AppError> {
    let claim_data = claims.into_inner();
    let dto = GetStickersByUser {
        user_id: claim_data.sub,
    };

    let stickers = postgres::stickers::get_stickers_by_user(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "stickers": stickers })))
}

#[has_permissions("stickers:get")]
pub async fn get_available_stickers(
    state: Data<AppState>,
    claims: ReqData<Claims>,
) -> actix_web::Result<HttpResponse, AppError> {
    let claim_data = claims.into_inner();
    let dto = GetAvailableStickers {
        user_id: claim_data.sub,
    };

    let stickers = postgres::stickers::get_available_stickers(&state.storage_layer.pg, dto)
        .await
        .map_err(|e| {
            log::error!("{e}");
            AppError::InternalServerError
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "stickers": stickers })))
}

#[has_permissions("stickers:edit")]
pub async fn edit_sticker(
    state: Data<AppState>,
    claims: ReqData<Claims>,
    sticker: Path<String>,
    data: Json<EditStickerRequest>,
) -> actix_web::Result<HttpResponse, AppError> {
    let claim_data = claims.into_inner();
    let info = data.into_inner();
    let dto = EditSticker {
        id: sticker.into_inner(),
        user_id: claim_data.sub,
        visibility: info.visibility,
        friendly_name: info.friendly_name,
    };

    match postgres::stickers::edit_sticker(&state.storage_layer.pg, dto)
        .await
        .map_err(|e| {
            log::error!("{e}");
            AppError::InternalServerError
        })? {
        1 => Ok(HttpResponse::Ok().json(serde_json::json!({"msg": "successfully edited sticker"}))),
        _ => Err(AppError::NotFound),
    }
}

#[has_permissions("stickers:delete")]
pub async fn delete_sticker(
    state: Data<AppState>,
    claims: ReqData<Claims>,
    sticker: Path<String>,
) -> actix_web::Result<HttpResponse, AppError> {
    let claim_data = claims.into_inner();
    let dto = DeleteSticker {
        id: sticker.into_inner(),
        user_id: claim_data.sub,
    };
    match postgres::stickers::delete_sticker(&state.storage_layer.pg, dto)
        .await
        .map_err(|_| AppError::InternalServerError)?
    {
        1 => Ok(HttpResponse::NoContent().finish()),
        _ => Err(AppError::NotFound),
    }
}
