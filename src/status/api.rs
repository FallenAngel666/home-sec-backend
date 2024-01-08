use uuid::Uuid;
use crate::common::routers::{CREATED_RESPONSE, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};
use crate::status::service;
use crate::status::service::ServiceResponse;
use crate::common::util::{get_id, get_status_request_body};

pub fn log_status(request: &str) -> (String, String) {
    match get_status_request_body(&request) {
        Ok(status) => {
            match service::log_status(&status) {
                ServiceResponse::Ok(s) => (CREATED_RESPONSE.to_string(), serde_json::to_string(&s).unwrap()),
                ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
                _ => (INTERNAL_ERROR.to_string(), "Something unexpected happened.".to_string())
            }
        }
        err => (INTERNAL_ERROR.to_string(), format!("Internal error: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_status_by_id(request: &str) -> (String, String) {
    match get_id(&request).parse::<Uuid>() {
        Ok(id) => {
            match service::get_status_by_id(&id) {
                ServiceResponse::Ok(s) => (OK_RESPONSE.to_string(), serde_json::to_string(&s).unwrap()),
                ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
                ServiceResponse::NotFound => (NOT_FOUND.to_string(), format!("Cannot find status with id {}", id))
            }
        }
        err => (INTERNAL_ERROR.to_string(), format!("Internal error: {}", err.err().unwrap().to_string())),
    }
}

pub fn get_all_statuses(_request: &str) -> (String, String) {
    match service::get_all_statuses() {
        ServiceResponse::Ok(s) => (OK_RESPONSE.to_string(), serde_json::to_string(&s).unwrap()),
        ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
        _ => (INTERNAL_ERROR.to_string(), "Something unexpected happened.".to_string())
    }
}

pub fn delete_status_by_id(request: &str) -> (String, String) {
    match get_id(&request).parse::<Uuid>() {
        Ok(id) => {
            match service::delete_status_by_id(&id) {
                ServiceResponse::Ok(s) => (OK_RESPONSE.to_string(), serde_json::to_string(&s).unwrap()),
                ServiceResponse::Error(err) => (INTERNAL_ERROR.to_string(), err),
                ServiceResponse::NotFound => (NOT_FOUND.to_string(), format!("Cannot find status with id {}", id))
            }
        }
        err => (INTERNAL_ERROR.to_string(), format!("Internal error: {}", err.err().unwrap().to_string())),
    }
}