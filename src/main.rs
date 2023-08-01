use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use app::{guards, routes, state::AppState};

mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("error loading environment variables");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = web::Data::new(AppState::new("milkandmocha").await);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(guards::jwt_guard);

        App::new()
            .wrap(Logger::default())
            .wrap(auth)
            .app_data(state.clone())
            .service(web::scope("/api").configure(routes::config))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
