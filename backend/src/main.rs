//! Entrypoint for the backend server application.

use actix_web::{web, App, HttpServer};

use project_m::income;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and start the server
    HttpServer::new(|| {
        App::new()
            // Limit the maximum amount of data the server will accept
            .app_data(web::JsonConfig::default().limit(4096))
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
