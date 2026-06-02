use lapin::{Connection, ConnectionProperties};

pub async fn connect(rabbitmq_url: &str) -> Result<Connection, lapin::Error> {
    let conn = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;
    Ok(conn)
}
