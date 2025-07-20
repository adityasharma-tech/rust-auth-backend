use dotenvy::dotenv;
use std::env;

pub struct Environment {
    pub database_url: String,
    pub rabbitmq_host: String,
    pub rabbitmq_password: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_username: String,
    pub access_secret_key: String,
    pub refresh_secret_key: String
}

pub fn get_env() -> Environment {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let rabbitmq_host = env::var("RABBITMQ_HOST").unwrap();
    let rabbitmq_password = env::var("RABBITMQ_PASSWORD").unwrap();
    let rabbitmq_port = env::var("RABBITMQ_PORT").unwrap().parse::<u16>().unwrap();
    let rabbitmq_username = env::var("RABBITMQ_USERNAME").unwrap();

    let access_secret_key = env::var("ACCESS_SECRET_KEY").unwrap();
    let refresh_secret_key = env::var("REFRESH_SECRET_KEY").unwrap();

    Environment {
        database_url,
        rabbitmq_password,
        rabbitmq_host,
        rabbitmq_port,
        refresh_secret_key,
        access_secret_key,
        rabbitmq_username,
    }
}
