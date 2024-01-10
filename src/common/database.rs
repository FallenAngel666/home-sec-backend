use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use crate::common::env::DB_URL;

pub enum DBResponse<T> {
    Ok(T),
    NotFound,
    Error(String),
}

pub fn create_status_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS status (
            id uuid PRIMARY KEY,
            client_id uuid NOT NULL,
            title VARCHAR NOT NULL,
            status VARCHAR NOT NULL,
            info VARCHAR
        )
    "
    )?;
    Ok(())
}