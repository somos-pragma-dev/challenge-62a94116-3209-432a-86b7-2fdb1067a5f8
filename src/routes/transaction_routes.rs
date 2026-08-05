use actix_web::{web, Scope};
use crate::handlers::create_transaction;

pub fn transaction_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/transactions")
       .route(web::post().to(create_transaction)))
}