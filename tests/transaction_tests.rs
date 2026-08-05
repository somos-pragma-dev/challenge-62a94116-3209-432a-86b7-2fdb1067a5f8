use actix_web::{test, App, http::Method};
use serde_json::json;

#[actix_rt::test]
async fn test_create_transaction() {
    let app = test::init_service(
        App::new().service(create_transaction)
    ).await;
    let req = test::TestRequest::new()
       .method(Method::POST)
       .uri("/transactions")
       .set_json(&json!({
            "amount": 100.0,
            "date": "2021-01-01T12:00:00",
            "transaction_type": "deposit",
            "status": "pending"
        }))
       .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}