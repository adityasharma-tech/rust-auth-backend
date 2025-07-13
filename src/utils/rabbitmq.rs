use crate::utils::env::get_env;
use actix_web::cookie::time;
use actix_web::error::HttpError;
use amqprs::BasicProperties;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{
    BasicPublishArguments, ExchangeDeclareArguments, ExchangeType, QueueBindArguments,
    QueueDeclareArguments,
};
use amqprs::connection::{Connection, OpenConnectionArguments};

pub async fn create_rmq_connection() -> Result<Connection, String> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        get_env().rabbitmq_host.as_str(),
        get_env().rabbitmq_port,
        get_env().rabbitmq_username.as_str(),
        get_env().rabbitmq_password.as_str(),
    ))
    .await
    .map_err(|_| "Failed to create connection to queue_server")?;

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .map_err(|_| "Failed to register callback.")?;

    Ok(connection)
}

pub async fn publish_message(exchange_name: &str, content: Vec<u8>) -> Result<(), String> {
    let connection = create_rmq_connection().await?;

    let channel = connection
        .open_channel(None)
        .await
        .map_err(|_| "Failed to create channel")?;

    channel
        .register_callback(DefaultChannelCallback)
        .await
        .map_err(|_| "Failed to register callback for channel")?;

    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::default())
        .await
        .map_err(|_| "Failed to declare queue for the default channel")?
        .unwrap();

    let routing_key = "";

    let mut exchange_arg: ExchangeDeclareArguments =
        ExchangeDeclareArguments::of_type(exchange_name, ExchangeType::Fanout);
    exchange_arg.durable(true);
    exchange_arg.no_wait(true);

    channel
        .exchange_declare(exchange_arg)
        .await
        .map_err(|_| "Failed to declare exchange")?;

    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .map_err(|_| "Failed to bind queue")?;

    let args = BasicPublishArguments::new(exchange_name, routing_key);

    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await
        .map_err(|_| {
            format!(
                "Failed to publish message to queue_name: {}, exchange_name: {}, routing_key: {}",
                &queue_name, exchange_name, routing_key
            )
        })?;

    channel
        .close()
        .await
        .map_err(|_| "Failed to close channel")?;
    connection
        .close()
        .await
        .map_err(|_| "Failed to close connection")?;

    Ok(())
}
