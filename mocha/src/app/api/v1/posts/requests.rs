use serde::{Deserialize, Serialize};

use crate::app::types::AssetVisibility;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditStickerRequest {
    pub friendly_name: String,
    pub visibility: AssetVisibility,
}
