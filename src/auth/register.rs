use crate::database::db::get_db_connection;
use crate::models::upgrade::{LastLoginMethod, NewUser, User};
use crate::schema::upgrade::users::dsl::*;
use crate::services::rabbitmq::publish_message;
use crate::utils::response::ApiResponse;
use crate::utils::types::PgPool;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use chrono::Local;
use diesel::QueryDsl;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use validator::Validate;

#[derive(Serialize, Validate, Debug, Deserialize)]
pub struct RegisterUserBody {
    #[validate(length(min = 3, max = 30))]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(length(min = 2, max = 30))]
    #[serde(rename = "lastName")]
    last_name: String,
    #[validate(email())]
    email: String,
    #[validate(length(min = 3, max = 16))]
    password: String,
    #[validate(length(min = 3, max = 20))]
    username: String,
}

pub async fn register_user(pool: PgPool, body: Json<RegisterUserBody>) -> impl Responder {
    let user = body.0;

    let connection = &mut pool.get().unwrap();

    let result = users
        .filter(email.eq(&user.email).or(username.eq(&user.username)))
        .limit(1)
        .select(User::as_select())
        .load(connection)
        .unwrap();

    if result.len() > 0 {
        return HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(ApiResponse {
                success: false,
                status_code: 400,
                data: Null,
                message: String::from("user already exists with same email or password"),
            });
    }

    let hashed_password = bcrypt::hash(user.password, 10).map_err(|_| "Some error occurred");

    let insert_result = insert_into(users)
        .values(NewUser {
            email: user.email,
            password_hash: hashed_password.unwrap(),
            phone_number: String::from(""),
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            updated_at: Some(Local::now().naive_local()),
            last_login_method: Some(LastLoginMethod::EmailPassword)
        })
        .returning(User::as_select())
        .get_results(connection);

    if insert_result.is_err() {
        HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(ApiResponse {
                success: false,
                status_code: 400,
                data: Null,
                message: String::from("Failed to register new user"),
            });
        panic!("{:?}", insert_result.err());
    }

    let otp = rand::thread_rng().gen_range(100000..999999);

    let publish_result = publish_message(
        "otp",
        String::from(format!("\"otp\": {}, \"transport_method\": \"email\", \"identifier\": \"{}\"", otp, insert_result.unwrap()[0].email)).into_bytes(),
    )
    .await;

    match publish_result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            status_code: 200,
            data: Null,
            message: String::from("User registered successfully"),
        }),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(ApiResponse {
                success: false,
                status_code: 400,
                data: Null,
                message: String::from("Failed to send otp"),
            }),
    }
}
