use crate::status::repository::{Status};

pub enum ServiceResponse<T> {
    Ok(T),
    NotFound,
    Error(String)
}

pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

pub fn get_status_request_body(request: &str) -> Result<Status, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}