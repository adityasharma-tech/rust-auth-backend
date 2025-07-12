use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use crate::utils::response::ApiResponse;

use validator::{Validate};
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginUserBody {
    #[validate(email)]
    email: String,
    #[validate(length(min=3, max=16))]
    password: String,
}

pub struct LoginUserResponse {

}

pub async fn login_user(body: Json<LoginUserBody>) -> impl Responder {
    let data = body.0;

    HttpResponse::Ok().json(ApiResponse { success: true, status_code: 200, data: Null, message: "Success".to_string() })
}