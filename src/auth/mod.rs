use actix_web::Scope;
use actix_web::web::{post, scope};

mod login;
mod register;

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/login", post().to(login::login_user))
        .route("/register", post().to(register::register_user))
}
