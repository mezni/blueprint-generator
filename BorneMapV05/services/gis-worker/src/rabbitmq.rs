use tracing::info;

pub fn check_rabbitmq_config(queue_name: &str) {
    info!(
        "RABBITMQ_QUEUE_GIS_SYNC is set to '{}', but RabbitMQ mode is not implemented in v1. Falling back to DB polling.",
        queue_name
    );
}
