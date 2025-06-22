mod database;

mod auth_utils {
    pub fn login(crdentials:models::Credentials) {
        crate::database::get_user();
    }
    
    fn logout() {
        
    }
    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::models::Credentials;
use database::connect_to_database;
use database::Status;

pub fn authenticate(credentails:Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(credentails);
    }
}
