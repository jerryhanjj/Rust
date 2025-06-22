use rand::prelude::*;

mod database;
mod auth_utils;

// Re-export the necessary items for external use
// 使用重导出技术，使得authenticate作为api接口，其入参需要被重导出作为公共使用，而不是作为私有
pub use auth_utils::models::Credentials;

use database::connect_to_database;
use database::Status;

pub fn authenticate(credentails:Credentials) {
    let timout = rand::rng().random_range(100..=500);
    println!("Authenticating with a timeout of {timout} seconds...");
    if let Status::Connected = connect_to_database() {
        auth_utils::login(credentails);
    }
}
