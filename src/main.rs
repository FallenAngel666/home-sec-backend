mod status;
mod common;

use std::net::TcpListener;
use crate::common::routers::main_route;
use crate::common::database::create_status_database;

fn main() {
    //Set Database
    if let Err(err) = create_status_database() {
        println!("Error setting database: {}", err);
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
