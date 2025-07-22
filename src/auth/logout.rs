use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::cookie::{time, Cookie, SameSite};
use serde_json::Value::Null;
use crate::utils::response::ApiResponse;

pub async fn logout_user(req: HttpRequest) -> impl Responder {
    let access_token_cookie = Cookie::build("__Secure-rfc_access", "")
        .domain("adityasharma.tech")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::seconds(0))
        .same_site(SameSite::None)
        .finish();
    let refresh_token_cookie = Cookie::build("__Secure-rfc_refresh", "")
        .domain("adityasharma.tech")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::seconds(0))
        .same_site(SameSite::None)
        .finish();

    HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Expires", "0"))
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResponse {
            success: true,
            status_code: 200,
            data: Null,
            message: "Logout Success".to_string(),
        })
}