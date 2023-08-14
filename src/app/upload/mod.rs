pub mod files {
    use std::error::Error;
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};

    use crate::app::{dto::AssetBackend, errors::AppError};

    // TODO: Remove unwrap when extracing file_name from content_type and add some safeguards
    pub async fn save_file_fs(mut payload: Multipart) -> Result<(), Box<dyn Error + Send + Sync>> {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let name = field.name();
            log::debug!("{name}");
            let content_type = field.content_disposition();
            log::debug!("{content_type}");
            let file_name = content_type.get_filename().unwrap();

            let filepath = format!("./src/app/assets/{file_name}");

            let mut f = web::block(|| std::fs::File::create(filepath)).await??;

            while let Some(chunk) = field.next().await {
                if chunk.is_err() {
                    Err(AppError::InternalServerError)?
                };
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }
        }
        Ok(())
    }

    pub async fn save_file(
        backend: AssetBackend,
        payload: Multipart,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match backend {
            AssetBackend::Fs => save_file_fs(payload).await,
            AssetBackend::Aws => unimplemented!(),
            AssetBackend::Gcp => unimplemented!(),
            AssetBackend::Azure => unimplemented!(),
        }
    }
}
