pub mod files {
    use std::error::Error;
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};

    use crate::app::{errors::AppError, types::AssetBackend, util};

    pub struct AssetUpload {
        pub file_path: String,
        pub friendly_name: String,
    }

    async fn save_assets_fs(
        mut payload: Multipart,
    ) -> Result<Vec<AssetUpload>, Box<dyn Error + Send + Sync>> {
        let uploads = loop {
            let mut file_paths = Vec::<AssetUpload>::new();
            if let Ok(Some(mut field)) = payload.try_next().await {
                if let (Some(file_name), Some(friendly_name)) = (
                    field.content_disposition().get_filename(),
                    field.content_disposition().get_name(),
                ) {
                    let random_prefix = util::rng::random_string(12);
                    let filepath = format!("./assets/{random_prefix}-{file_name}");
                    file_paths.push(AssetUpload {
                        file_path: filepath.to_owned(),
                        friendly_name: friendly_name.to_owned(),
                    });
                    let copy = filepath.clone();
                    let mut f = web::block(move || std::fs::File::create(copy)).await??;
                    while let Some(chunk) = field.next().await {
                        match chunk {
                            Ok(data) => {
                                f = web::block(move || f.write_all(&data).map(|_| f)).await??
                            }
                            Err(e) => {
                                log::error!("error uploading image: {e}");
                                Err(AppError::InternalServerError)?;
                            }
                        };
                    }
                }
            } else {
                break file_paths;
            };
        };
        Ok(uploads)
    }

    pub async fn save_assets(
        backend: AssetBackend,
        payload: Multipart,
    ) -> Result<Vec<AssetUpload>, Box<dyn Error + Send + Sync>> {
        match backend {
            AssetBackend::Fs => save_assets_fs(payload).await,
            AssetBackend::Aws => unimplemented!(),
            AssetBackend::Gcp => unimplemented!(),
            AssetBackend::Azure => unimplemented!(),
        }
    }
}
