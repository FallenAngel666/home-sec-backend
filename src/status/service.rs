use uuid::Uuid;
use crate::client;
use crate::client::repository::Client;
use crate::common::database::DBResponse;
use crate::common::util::ServiceResponse;
use crate::status::repository;
use crate::status::repository::Status;


pub fn log_status(status: &Status) -> ServiceResponse<Status> {
    let client = Client {
        id: status.client_id,
        name: "UNDEFINED".to_string()
    };
    match client::service::create_new_client_or_return_existing_one(&client) {
        ServiceResponse::Ok(_) => {}
        ServiceResponse::NotFound => {return ServiceResponse::Error("Could not find created client entity for new status.".to_string())}
        ServiceResponse::Error(err) => {return ServiceResponse::Error(err)}
    }

    match repository::save_status(status) {
        DBResponse::Ok(id) => {
            match repository::get_status(&id) {
                DBResponse::Ok(s) => { ServiceResponse::Ok(s)}
                DBResponse::NotFound => { ServiceResponse::Error("Could not find created status entity.".to_string())}
                DBResponse::Error(err) => { ServiceResponse::Error(err)}
            }
        }
        DBResponse::Error(err) => { ServiceResponse::Error(err)}
        _ => ServiceResponse::Error("Something unexpected happened.".to_string())
    }
}

pub fn get_status_by_id(id: &Uuid) -> ServiceResponse<Status> {
    match repository::get_status(id) {
        DBResponse::Ok(s) => { ServiceResponse::Ok(s)}
        DBResponse::NotFound => { ServiceResponse::NotFound}
        DBResponse::Error(err) => { ServiceResponse::Error(err)}
    }
}

pub fn get_all_statuses() -> ServiceResponse<Vec<Status>> {
    match repository::get_all_statuses() {
        DBResponse::Ok(s) => { ServiceResponse::Ok(s)}
        DBResponse::Error(err) => { ServiceResponse::Error(err)}
        _ => ServiceResponse::Error("Something unexpected happened.".to_string())
    }
}

pub fn delete_status_by_id(id: &Uuid) -> ServiceResponse<Status> {
    let exiting_status = match get_status_by_id(id) {
        ServiceResponse::Ok(s) => {s}
        ServiceResponse::NotFound => {return ServiceResponse::NotFound}
        ServiceResponse::Error(err) => {return ServiceResponse::Error(err)}
    };

    match repository::delete_status(id) {
        DBResponse::Ok(_) => {ServiceResponse::Ok(exiting_status)}
        DBResponse::NotFound => {ServiceResponse::NotFound}
        DBResponse::Error(err) => {ServiceResponse::Error(err)}
    }
}