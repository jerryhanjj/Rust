mod database;
mod auth_utils;

use auth_utils::models::Credentials;
use database::connect_to_database;
use database::Status;

pub fn authenticate(credentails:Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(credentails);
    }
}
