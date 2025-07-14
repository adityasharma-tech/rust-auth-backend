use crate::models::upgrade::{LastLoginMethod, Session, User};
use crate::schema::upgrade::session::dsl::session;
use crate::schema::upgrade::session::{session_id, user_id};
use crate::schema::upgrade::session::{expire_at, invalid};
use crate::utils::response::ApiResponse;
use crate::utils::types::PgPool;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use bcrypt::verify;
use diesel::dsl::update;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use uaparser::UserAgentParser;
use validator::Validate;
use crate::utils::fetch_location_data;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginUserBody {
    #[validate(length(min = 3))]
    identifier: String,
    #[validate(length(min = 3))]
    password: String,
}

pub async fn login_user(pool: PgPool, body: Json<LoginUserBody>) -> impl Responder {
    use crate::schema::upgrade::users::dsl::*;
    let payload = body.0;

    let mut connection = pool.get().unwrap();

    let result = users
        .filter(
            email
                .eq(&payload.identifier)
                .or(username.eq(&payload.identifier)),
        )
        .limit(1)
        .select(User::as_select())
        .load(&mut connection)
        .map_err(|_| "")
        .unwrap();

    if result.len() <= 0 {
        return HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(ApiResponse {
                success: false,
                status_code: 404,
                data: Null,
                message: String::from("invalid username/email or password"),
            });
    }

    let is_password_correct = verify(payload.password, result[0].password_hash.as_str()).unwrap();

    if !is_password_correct {
        return HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(ApiResponse {
                success: false,
                status_code: 404,
                data: Null,
                message: String::from("invalid username/email or password"),
            });
    }

    update(users)
        .filter(id.eq(result[0].id))
        .set(
            last_login_method.eq::<Option<LastLoginMethod>>(Some(LastLoginMethod::EmailPassword))
        )
        .execute(&mut connection)
        .expect("Error updating last login method");

    let active_sessions = session
        .filter(user_id.eq(result[0].id))
        .filter(invalid.eq(false))
        .select(Session::as_select())
        .filter(expire_at.gt(diesel::dsl::now))
        .load(&mut connection)
        .unwrap();

    if active_sessions.len() >= 5 {
        update(session)
            .filter(session_id.eq(active_sessions[0].session_id))
            .set((invalid.eq(true), expire_at.eq(diesel::dsl::now)))
            .execute(&mut connection)
            .unwrap();
    }

    let location_result = fetch_location_data(String::new()).await;

    let ua_parser = UserAgentParser::from_yaml("../../ua_regexes.yaml").expect_err("Parser creation failed!");

    match location_result {
        Ok(location_data) => {

        },
        Err(_) => {}
    }

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        status_code: 200,
        data: Null,
        message: "Success".to_string(),
    })
}
