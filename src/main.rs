use actix_web::{web, App, HttpServer};
use app::{controller, state::AppState};

mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("error loading environment variables");

    let state = web::Data::new(AppState::new("milkandmocha").await);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").configure(controller::config))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
