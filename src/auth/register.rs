use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::{Local};
use diesel::dsl::insert_into;
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use validator::Validate;
use crate::database::db::establish_connection;
use diesel::prelude::*;
use crate::models::upgrade::{NewUser, User};
use crate::schema::upgrade::users::dsl::*;
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

    let connection = &mut establish_connection();

    let result = users.filter(email.eq(&user.email).or(username.eq(&user.username))).limit(1).select(User::as_select()).load(connection).expect("Error loading posts");

    if result.len() > 0 {
        return HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(ApiResponse { success: false, status_code: 400, data: Null, message: String::from("user already exists with same email or password") });
    }

    let hashed_password = bcrypt::hash(user.password, 10);

    let insert_result = insert_into(users).values(NewUser { email: user.email, password_hash: hashed_password.unwrap(), phone_number: String::from(""), first_name: user.first_name, last_name: user.last_name, username: user.username, updated_at: Some(Local::now().naive_local()) }).execute(connection);

    if insert_result.is_err() {
        return HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(ApiResponse { success: false, status_code: 500, data: Null, message: String::from("Failed to register new user") });
    }

    // TODO: Send otp to the mail box and get email_id in return

    HttpResponse::Ok().json(ApiResponse { success: true, status_code: 200, data: Null, message: String::from("User registered successfully") })
}
