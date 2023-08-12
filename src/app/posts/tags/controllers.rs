use actix_web::web::Data;
use actix_web::web::Json;

use crate::app::dto::CreateTag;
use crate::app::state::AppState;

pub async fn create_tag(state: Data<AppState>, data: Json<CreateTag>) {}
