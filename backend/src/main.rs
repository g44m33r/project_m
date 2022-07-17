//! Entrypoint for the backend server application.

use actix_web::{middleware, web, App, HttpServer};
use log::info;

use project_m::income;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure the logger
    let env = env_logger::Env::default()
        .filter_or("DEFAULT_LOG_LEVEL", "debug")
        .write_style_or("DEFAULT_WRITE_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Starting the server on :8080");

    // Create and start the server
    HttpServer::new(|| {
        App::new()
            // Limit the maximum amount of data the server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            // Enable logging
            .wrap(middleware::Logger::default())
            // Define routes and handlers
            .service(
                web::resource(income::API_URL_INCOME)
                    .route(web::get().to(income::get_income))
                    .route(web::post().to(income::add_income))
                    .route(web::delete().to(income::remove_income)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
