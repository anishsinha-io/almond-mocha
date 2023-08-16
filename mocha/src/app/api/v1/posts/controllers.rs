use actix_multipart::Multipart;
use actix_web::{
    web::{Data, ReqData},
    HttpResponse,
};

use crate::app::{
    auth::tokens::Claims,
    dto::stickers::{CreateSticker, CreateStickers},
    errors::AppError,
    state::AppState,
    storage::postgres,
    types::AssetVisibility,
    upload,
};

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

            // This is guaranteed to be Ok because if there were an error, map_err would propagate
            // an internal server error, so calling unwrap is perfectly fine.
            let new_sticker_ids = postgres::stickers::create_stickers(&state.storage_layer.pg, dto)
                .await
                .map_err(|_| AppError::InternalServerError);

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "msg":
                    format!(
                        "successfully created new sticker(s): {:#?}",
                        new_sticker_ids.unwrap()
                    )
            })))
        }
        Err(e) => {
            log::error!("error creating sticker: {e}");
            Err(AppError::InternalServerError)
        }
    }
}
