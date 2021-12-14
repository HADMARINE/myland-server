use super::structs::stats::{lands::Seoul, User};

pub static waiting_users: Vec<String> = Vec::new();
pub static registered_users: Vec<User<Seoul>> = Vec::new();
pub static ready_count: u8 = 0;
// pub static client_ready_list: Vec<User
