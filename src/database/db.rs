use diesel::PgConnection;

use crate::utils::env::get_env;

use diesel::r2d2::{ConnectionManager, Pool};
pub fn get_db_connection() -> Result<Pool<ConnectionManager<PgConnection>>, String> {
    let connection_manager = ConnectionManager::<PgConnection>::new(get_env().database_url);
    match Pool::builder().build(connection_manager) {
        Ok(p) => Ok(p),
        Err(_) => Err(String::from("Failed to make connection with database")),
    }
}
