mod status;
mod common;
mod client;

use std::net::TcpListener;
use crate::common::routers::main_route;
use crate::common::database::{create_clients_database, create_status_database};

fn main() {
    //Set Databases
    if let Err(err) = create_status_database() {
        println!("Error setting status database: {}", err);
        return;
    }
    if let Err(err) = create_clients_database() {
        println!("Error setting clients database: {}", err);
        return;
    }

    //start server and print port
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
