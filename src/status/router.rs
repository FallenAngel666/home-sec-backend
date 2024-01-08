use crate::common::routers::NOT_FOUND;
use crate::status::api::{delete_status_by_id, get_all_statuses, get_status_by_id, log_status};

pub fn route(request: &str) -> (String, String) {
    return match &*request {
        r if r.starts_with("POST /status") => log_status(r),
        r if r.starts_with("GET /status/") => get_status_by_id(r),
        r if r.starts_with("GET /status") => get_all_statuses(r),
        r if r.starts_with("DELETE /status/") => delete_status_by_id(r),
        _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
    };
}