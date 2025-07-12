use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use validator::Validate;
use crate::utils::response::ApiResponse;

#[derive(Serialize, Validate, Debug, Deserialize)]
pub struct RegisterUserBody {
    #[validate(length(min=3, max=30))]
    #[serde(rename="firstName")]
    first_name: String,
    #[validate(length(min=2, max=30))]
    #[serde(rename="lastName")]
    last_name: String,
    #[validate(email())]
    email: String,
    #[validate(length(min=3, max=16))]
    password: String,
    #[validate(length(min=3, max=20))]
    username: String
}

pub async fn register_user(body: Json<RegisterUserBody>) -> impl Responder {

    let user = body.0;



    HttpResponse::Ok().json(ApiResponse { success: true, status_code: 200, data: Null, message: String::from("User registered successfully") })
}