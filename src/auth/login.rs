use actix_web::{HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use bcrypt::{verify};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::dsl::update;
use diesel::expression::AsExpression;
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use crate::utils::response::ApiResponse;
use validator::{Validate};
use crate::database::db::establish_connection;
use crate::models::upgrade::{LastLoginMethod, User};
use crate::schema::upgrade::chats::user_id;
use crate::schema::upgrade::session::dsl::session;
use crate::schema::upgrade::session::{expire_at, invalid};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginUserBody {
    #[validate(length(min=3))]
    identifier: String,
    #[validate(length(min=3))]
    password: String
}

pub async fn login_user(body: Json<LoginUserBody>) -> impl Responder {
    use crate::schema::upgrade::users::dsl::*;
    let payload = body.0;

    let mut connection = establish_connection();

    let result = users.filter(email.eq(&payload.identifier).or(username.eq(&payload.identifier))).limit(1).select(User::as_select()).load(&mut connection).map_err(|_| "").unwrap();

    if result.len() <= 0 {
        return HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(ApiResponse { success: false, status_code: 404, data: Null, message: String::from("invalid username/email or password") });
    }

    let is_password_correct = verify(payload.password, result[0].password_hash.as_str()).unwrap();

    if !is_password_correct {
        return HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(ApiResponse { success: false, status_code: 404, data: Null, message: String::from("invalid username/email or password") });
    }

    update(users).filter(id.eq(result[0].id)).set((last_login_method.eq::<Option<LastLoginMethod>>(Some(LastLoginMethod::EmailPassword)))).execute(&mut connection).expect("Error updating last login method");

    let activeSessions = session.filter(user_id.eq(result[0].id)).filter(invalid.eq(false)).select().filter(expire_at.gt(diesel::dsl::now)).execute(&mut connection).expect("Failed to get sessions");

    HttpResponse::Ok().json(ApiResponse { success: true, status_code: 200, data: Null, message: "Success".to_string() })
}