use actix_web::Scope;
use actix_web::web::{get, post, scope};
use crate::middleware::auth::AuditDataBase;
use actix_web::dev::HttpServiceFactory;

mod login;
mod register;
mod logout;
mod sessions;

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/login", post().to(login::login_user))
        .route("/register", post().to(register::register_user))
}

pub fn secure_auth_routes() -> impl HttpServiceFactory {
    
    scope("/user")
        .wrap(AuditDataBase)
        .route("/logout", get().to(logout::logout_user))
}