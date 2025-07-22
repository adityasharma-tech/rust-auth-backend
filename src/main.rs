use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde_json;
use serde_json::Value::Null;

//...
mod auth;
mod database;
mod models;
mod schema;
mod utils;
mod validators;
mod services;
mod middleware;

//...
use crate::auth::auth_routes;
use crate::database::db::get_db_connection;

#[get("/healthCheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(utils::response::ApiResponse {
        message: String::from("Server is healthy"),
        data: Null,
        status_code: 200,
        success: true,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("Starting server on port {}", port);

    let pool = get_db_connection().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .service(auth_routes())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
