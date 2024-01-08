use postgres::{Client, NoTls};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::database::DBResponse;
use crate::common::env::DB_URL;

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub(crate) id: Option<Uuid>,
    pub(crate) title: String,
    pub(crate) status: String,
    pub(crate) info: String
}

pub fn save_status(status: &Status) -> DBResponse<Uuid> {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let id = Uuid::new_v4();
            client
                .execute(
                    "INSERT INTO status (id, title, status, info) VALUES ($1, $2, $3, $4)",
                    &[&id, &status.title, &status.status, &status.info],
                ).unwrap();
            DBResponse::Ok(id)
        }
        err => DBResponse::Error(format!("Could not save new status: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_status(id: &Uuid) -> DBResponse<Status> {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) =>
            match client.query_one("SELECT * FROM status WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let status = Status {
                        id: row.get(0),
                        title: row.get(1),
                        status: row.get(2),
                        info: row.get(3),
                    };
                    DBResponse::Ok(status)
                }
                _ => DBResponse::NotFound,
            }

        err => DBResponse::Error(format!("Could not save new status: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_all_statuses() -> DBResponse<Vec<Status>> {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut stati: Vec<Status> = Vec::new();

            for row in client.query("SELECT id, title, status, info FROM status", &[]).unwrap() {
                stati.push(Status {
                    id: row.get(0),
                    title: row.get(1),
                    status: row.get(2),
                    info: row.get(3),
                });
            }

            return DBResponse::Ok(stati);
        }
        err => DBResponse::Error(format!("Could not fetch all statuses: {}", err.err().unwrap().to_string())),
    }
}

pub fn delete_status(id: &Uuid) -> DBResponse<bool> {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let rows_affected = client.execute("DELETE FROM status WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return DBResponse::NotFound
            }

            return DBResponse::Ok(false);
        }
        err => DBResponse::Error(format!("Could not delete status with id: {}. Details: {}", id, err.err().unwrap().to_string())),
    }
}