pub fn login(crdentials:models::Credentials) {
    crate::database::get_user();
}

fn logout() {
    
}

pub mod models;