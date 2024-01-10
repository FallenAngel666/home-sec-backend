use postgres::NoTls;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::database::DBResponse;
use crate::common::env::DB_URL;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub(crate) id: Uuid,
    pub(crate) name: String,
}

pub fn save_client(c: &Client) -> DBResponse<Uuid> {
    match postgres::Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            client
                .execute(
                    "INSERT INTO clients (id, name) VALUES ($1, $2)",
                    &[&c.id, &c.name],
                ).unwrap();
            DBResponse::Ok(c.id)
        }
        err => DBResponse::Error(format!("Could not save new client: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_client(id: &Uuid) -> DBResponse<Client> {
    match postgres::Client::connect(DB_URL, NoTls) {
        Ok(mut client) =>
            match client.query_one("SELECT * FROM clients WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let c = Client {
                        id: row.get(0),
                        name: row.get(1),
                    };
                    DBResponse::Ok(c)
                }
                _ => DBResponse::NotFound,
            }

        err => DBResponse::Error(format!("Could not save new client: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_all_clients() -> DBResponse<Vec<Client>> {
    match postgres::Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut clients: Vec<Client> = Vec::new();

            for row in client.query("SELECT id, name FROM clients", &[]).unwrap() {
                clients.push(Client {
                    id: row.get(0),
                    name: row.get(1),
                });
            }

            return DBResponse::Ok(clients);
        }
        err => DBResponse::Error(format!("Could not fetch all clients: {}", err.err().unwrap().to_string())),
    }
}

pub fn delete_client(id: &Uuid) -> DBResponse<bool> {
    match postgres::Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let rows_affected = client.execute("DELETE FROM clients WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return DBResponse::NotFound
            }

            return DBResponse::Ok(false);
        }
        err => DBResponse::Error(format!("Could not delete clients with id: {}. Details: {}", id, err.err().unwrap().to_string())),
    }
}