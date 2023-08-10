use actix_web::web::{self, Data, ServiceConfig};

use super::{
    controllers::{login, register},
    state::AuthState,
};

pub fn config(cfg: &mut ServiceConfig) {
    let state = Data::new(AuthState::new());
    cfg.service(
        web::scope("/auth")
            .app_data(state)
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}
