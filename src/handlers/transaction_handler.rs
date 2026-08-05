use actix_web::{web, HttpResponse, Responder};
use crate::models::Transaction;

pub async fn create_transaction(info: web::Json<Transaction>) -> impl Responder {
    // Validación de datos
    if info.amount <= 0.0 {
        return HttpResponse::BadRequest().body("El monto debe ser positivo");
    }
    if!["deposit", "withdrawal"].contains(&info.transaction_type.as_str()) {
        return HttpResponse::BadRequest().body("Tipo de transacción inválido");
    }
    // Lógica para guardar la transacción en la base de datos
    HttpResponse::Ok().body("Transacción creada")
}