use serde_json;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::Value::Null;

//...
mod utils;
mod auth;
mod validators;
mod database;
mod schema;
mod models;

//...
use crate::auth::auth_routes;

#[get("/healthCheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(utils::response::ApiResponse {
        message: String::from("Server is healthy"),
        data: Null,
        status_code: 200,
        success: true
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("Starting server on port {}", port);

    HttpServer::new(|| App::new()
        .app_data(34)
            .service(health_check)
            .service(auth_routes())
        )
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

