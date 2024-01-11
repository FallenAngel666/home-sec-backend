use uuid::Uuid;
use crate::client::service;
use crate::common::routers::{INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};
use crate::common::util::{get_client_request_body, get_id, ServiceResponse};

pub fn get_all_clients(_request: &str) -> (String, String) {
    match service::get_all_clients() {
        ServiceResponse::Ok(c) => (OK_RESPONSE.to_string(), serde_json::to_string(&c).unwrap()),
        ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
        _ => (INTERNAL_ERROR.to_string(), "Something unexpected happened.".to_string())
    }
}

pub fn update_client(request: &str) -> (String, String) {
    match get_client_request_body(&request) {
        Ok(client) => {
            match service::update_client(&client) {
                ServiceResponse::Ok(c) => (OK_RESPONSE.to_string(), serde_json::to_string(&c).unwrap()),
                ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
                ServiceResponse::NotFound => (NOT_FOUND.to_string(), format!("Cannot find client with id {}", client.id))
            }
        }
        err => (INTERNAL_ERROR.to_string(), format!("Internal error: {}", err.err().unwrap().to_string())),
    }
}

pub fn delete_client_by_id(request: &str) -> (String, String) {
    match get_id(&request).parse::<Uuid>() {
        Ok(id) => {
            match service::delete_client_by_id(&id) {
                ServiceResponse::Ok(c) => (OK_RESPONSE.to_string(), serde_json::to_string(&c).unwrap()),
                ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
                ServiceResponse::NotFound => (NOT_FOUND.to_string(), format!("Cannot find client with id {}", id))
            }
        }
        err => (INTERNAL_ERROR.to_string(), format!("Internal error: {}", err.err().unwrap().to_string())),
    }
}