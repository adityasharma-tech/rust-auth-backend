use diesel::PgConnection;

pub type PgPool =
    actix_web::web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>;
