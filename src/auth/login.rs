use crate::models::upgrade::{AuthMethod, LastLoginMethod, NewSession, Roles, Session, User};
use crate::schema::upgrade::session::dsl::session;
use crate::schema::upgrade::session::{session_id, user_id};
use crate::schema::upgrade::session::{expire_at, invalid};
use crate::utils::response::ApiResponse;
use crate::utils::types::PgPool;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::cookie::{time, Cookie, SameSite};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::dsl::{insert_into, update};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use jsonwebtoken::{EncodingKey};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use uaparser::{Parser, UserAgentParser};
use validator::Validate;
use crate::utils::env::get_env;
use crate::utils::fetch_location_data;



#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginUserBody {
    #[validate(length(min = 3))]
    identifier: String,
    #[validate(length(min = 3))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    role: String,
    username: String,
    email_verified: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    id: String
}

pub async fn login_user(pool: PgPool, req: HttpRequest, body: Json<LoginUserBody>) -> impl Responder {
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

    let ip_addr = req.peer_addr().unwrap();

    let ua_parser = UserAgentParser::from_yaml("../../ua_regexes.yaml").unwrap();
    let user_agent = req.headers().get("user-agent").unwrap().to_str().unwrap();

    let ua  = ua_parser.parse(user_agent);

    let refresh_token_claim = RefreshTokenClaims {
        id: result[0].id.to_string()
    };

    let my_role = match result[0].role {
        Some(Roles::Admin) => String::from("admin"),
        Some(Roles::Viewer) => String::from("viewer"),
        Some(Roles::Streamer) => String::from("streamer"),
        _ => String::from("viewer")
    };

    let access_token_claim = AccessTokenClaims {
        id: result[0].id.to_string(),
        email: result[0].email.to_string(),
        username: result[0].username.to_string(),
        first_name: result[0].first_name.to_string(),
        last_name: result[0].last_name.to_string(),
        email_verified: result[0].email_verified,
        role: my_role
    };

    let refresh_token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &refresh_token_claim, &EncodingKey::from_secret(get_env().refresh_secret_key.as_ref())).unwrap();
    let access_token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &access_token_claim, &EncodingKey::from_secret(get_env().access_secret_key.as_ref())).unwrap();


    match location_result {
        Ok(location_data) => {
            insert_into(session)
                .values(NewSession {
                    country: location_data.country,
                    region: location_data.region,
                    city: location_data.city,
                    user_id: result[0].id,
                    auth_method: AuthMethod::EmailPassword,
                    ip_address: Some(ip_addr.ip().to_string()),
                    token: refresh_token.clone(),
                    user_agent: String::from(user_agent),
                    expire_at: Utc::now().naive_utc() + Duration::days(4),
                    languages: vec![],
                    os: Some(ua.os.family.to_string()),
                    platform: Some(ua.device.family.to_string()),
                    mobile: Some(ua.device.family.eq("mobile")),
                    timezone: Some(String::new()),
                    telecom: Some(String::new()),
                })
                .returning(Session::as_select())
                .get_result(&mut connection).expect("");
        },
        Err(_) => {
            insert_into(session)
                .values(NewSession {
                    country: Some(String::new()),
                    region: Some(String::new()),
                    city: Some(String::new()),
                    user_id: result[0].id,
                    auth_method: AuthMethod::EmailPassword,
                    ip_address: Some(ip_addr.ip().to_string()),
                    token: refresh_token.clone(),
                    user_agent: String::from(user_agent),
                    expire_at: Utc::now().naive_utc() + Duration::days(4),
                    languages: vec![],
                    os: Some(ua.os.family.to_string()),
                    platform: Some(ua.device.family.to_string()),
                    mobile: Some(ua.device.family.eq("mobile")),
                    timezone: Some(String::new()),
                    telecom: Some(String::new()),
                })
                .returning(Session::as_select())
                .get_result(&mut connection).expect("");
        }
    }

    let access_token_cookie = Cookie::build("__Secure-rfc_access", access_token)
        .domain("adityasharma.tech")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::days(4))
        .same_site(SameSite::None)
        .finish();
    let refresh_token_cookie = Cookie::build("__Secure-rfc_refresh", refresh_token)
        .domain("adityasharma.tech")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::days(4))
        .same_site(SameSite::None)
        .finish();

    HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResponse {
            success: true,
            status_code: 200,
            data: Null,
            message: "Success".to_string(),
    })
}
