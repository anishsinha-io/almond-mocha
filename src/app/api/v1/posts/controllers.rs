use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Form, Json},
    HttpResponse,
};

use crate::app::{
    dto::stickers::{CreateSticker, CreateStickerInfo},
    errors::AppError,
    state::AppState,
    upload,
};

pub async fn create_sticker(
    state: Data<AppState>,
    payload: Multipart,
    // data: Form<CreateStickerInfo>,
) -> actix_web::Result<HttpResponse, AppError> {
    let upload_status = upload::files::save_file_fs(payload).await;
    // let info = data.into_inner();

    match upload_status {
        Ok(_) => {
            //
            // let dto = CreateSticker {
            //     friendly_name: info.friendly_name,
            //     private: info.private,
            //     backend: state.config.asset_backend,
            //     file_path: "".to_owned(),
            // };
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("update_succeeded"))
        }
        Err(e) => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("update_failed {e}"))),
    }
}
