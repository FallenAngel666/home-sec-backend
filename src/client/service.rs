use uuid::Uuid;
use crate::client::repository::Client;
use crate::common::database::DBResponse;
use crate::common::util::{ServiceResponse};
use crate::client::repository;


pub fn create_new_client_or_return_existing_one(client: &Client) -> ServiceResponse<Client> {
    match repository::get_client(&client.id) {
        DBResponse::Ok(s) => { ServiceResponse::Ok(s) }
        DBResponse::NotFound => {
            match create_client(&client) {
                DBResponse::Ok(c) => { ServiceResponse::Ok(c) }
                DBResponse::NotFound => { ServiceResponse::Error("Could not find created client entity.".to_string()) }
                DBResponse::Error(err) => { ServiceResponse::Error(err) }
            }
        }
        DBResponse::Error(err) => { ServiceResponse::Error(err) }
    }
}

fn create_client(client: &Client) -> DBResponse<Client> {
    match repository::save_client(client) {
        DBResponse::Ok(id) => {
            repository::get_client(&id)
        }
        DBResponse::Error(err) => { DBResponse::Error(err)}
        _ => DBResponse::Error("Something unexpected happened.".to_string())
    }
}

pub fn get_client_by_id(id: &Uuid) -> ServiceResponse<Client> {
    match repository::get_client(id) {
        DBResponse::Ok(c) => { ServiceResponse::Ok(c)}
        DBResponse::NotFound => { ServiceResponse::NotFound}
        DBResponse::Error(err) => { ServiceResponse::Error(err)}
    }
}

pub fn get_all_clients() -> ServiceResponse<Vec<Client>> {
    match repository::get_all_clients() {
        DBResponse::Ok(c) => { ServiceResponse::Ok(c)}
        DBResponse::Error(err) => { ServiceResponse::Error(err)}
        _ => ServiceResponse::Error("Something unexpected happened.".to_string())
    }
}

// no json-patch needed - too less fields
pub fn update_client(client: &Client) -> ServiceResponse<Client> {
    match get_client_by_id(&client.id) {
        ServiceResponse::Ok(_) => {}
        ServiceResponse::NotFound => {return ServiceResponse::NotFound}
        ServiceResponse::Error(err) => {return ServiceResponse::Error(err)}
    };

    match repository::update_client(client) {
        DBResponse::Ok(_) => {ServiceResponse::Ok(
            match get_client_by_id(&client.id) {
                ServiceResponse::Ok(c) => {c}
                ServiceResponse::NotFound => { return ServiceResponse::NotFound}
                ServiceResponse::Error(err) => { return ServiceResponse::Error(err)}
            }
        )}
        DBResponse::NotFound => {ServiceResponse::NotFound}
        DBResponse::Error(err) => {ServiceResponse::Error(err)}
    }
}

pub fn delete_client_by_id(id: &Uuid) -> ServiceResponse<Client> {
    let exiting_client = match get_client_by_id(id) {
        ServiceResponse::Ok(c) => {c}
        ServiceResponse::NotFound => {return ServiceResponse::NotFound}
        ServiceResponse::Error(err) => {return ServiceResponse::Error(err)}
    };

    match repository::delete_client(id) {
        DBResponse::Ok(_) => {ServiceResponse::Ok(exiting_client)}
        DBResponse::NotFound => {ServiceResponse::NotFound}
        DBResponse::Error(err) => {ServiceResponse::Error(err)}
    }
}