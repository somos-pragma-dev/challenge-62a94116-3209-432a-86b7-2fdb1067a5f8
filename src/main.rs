mod handlers;
mod routes;
mod services;
mod models;
mod database;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
           .configure(routes::transaction_routes)
    })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}