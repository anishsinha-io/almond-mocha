use actix_web::{
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use app::{routes, state::AppState};

use crate::app::errors::AppError;

mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("error loading environment variables");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let state = web::Data::new(AppState::new("mocha").await);

    log::info!("brewing mocha with almond milk...");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new().handler(actix_web::http::StatusCode::UNAUTHORIZED, |_res| {
                    Err(AppError::Unauthorized)?
                }),
            )
            .app_data(state.clone())
            .service(web::scope("/api").configure(routes::config))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
