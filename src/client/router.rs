use crate::client::api::{delete_client_by_id, get_all_clients};
use crate::common::routers::NOT_FOUND;

pub fn route(request: &str) -> (String, String) {
    return match &*request {
        r if r.starts_with("GET /clients") => get_all_clients(r),
        r if r.starts_with("DELETE /clients/") => delete_client_by_id(r),
        _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
    };
}