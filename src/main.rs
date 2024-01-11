mod status;
mod common;
mod client;

use std::net::TcpListener;
use postgres::Error;
use crate::common::routers::main_route;
use crate::common::database::{create_clients_database, create_status_database};

fn main() {
    setup_databases().expect("Error setting database");
    startup_server();
}

fn setup_databases() -> Result<(), Error> {
    return create_clients_database()
        .and(create_status_database());
}

fn startup_server() {
    let listener = TcpListener::bind("0.0.0.0:8080".to_string()).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                main_route(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}