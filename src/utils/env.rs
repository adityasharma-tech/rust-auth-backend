use std::env;
use dotenvy::dotenv;

pub struct Environment {
    pub database_url: String,
    pub rabbitmq_host: String,
    pub rabbitmq_password: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_username: String
}

pub fn get_env() -> Environment {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let rabbitmq_host= env::var("RABBITMQ_HOST").unwrap();
    let rabbitmq_password = env::var("RABBITMQ_PASSWORD").unwrap();
    let rabbitmq_port = env::var("RABBITMQ_PORT").unwrap().parse::<u16>().unwrap();
    let rabbitmq_username = env::var("RABBITMQ_USERNAME").unwrap();

    Environment {
        database_url,
        rabbitmq_password,
        rabbitmq_host,
        rabbitmq_port,
        rabbitmq_username
    }
}