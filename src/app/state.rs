use super::config::{Config, StorageLayer};

pub struct AppState {
    pub config: Config,
    pub storage_layer: StorageLayer,
}

impl AppState {
    pub async fn new(name: &str) -> Self {
        let config = Config::new(name).expect("error generating app configuration");
        let storage_layer = StorageLayer::new()
            .await
            .expect("error initializing storage backend");

        Self {
            config,
            storage_layer,
        }
    }
}
