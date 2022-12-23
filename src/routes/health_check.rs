use actix_web::{HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Portfolio-Server-v1 : Working")
}