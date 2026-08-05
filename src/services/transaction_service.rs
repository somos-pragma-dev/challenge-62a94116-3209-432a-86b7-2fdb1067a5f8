use crate::models::Transaction;

pub async fn save_transaction(transaction: Transaction) -> Result<Transaction, String> {
    // Lógica para guardar la transacción en la base de datos
    Ok(transaction)
}