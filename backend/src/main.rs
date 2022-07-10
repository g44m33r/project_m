use actix_web::{web, App, HttpServer};

mod expense;
mod income;
mod recurring;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(income::add_income))
        .bind(("127.0.0.1", "8080"))?
        .run()
        .await
}
