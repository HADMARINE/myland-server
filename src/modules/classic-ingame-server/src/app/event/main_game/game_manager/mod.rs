use super::structs::stats::{lands::Seoul, User};

pub static WAITING_USERS: Vec<String> = Vec::new();
pub static REGISTERED_USERS: Vec<User<Seoul>> = Vec::new();
pub static READY_COUNT: u8 = 0;
pub static CLIENT_READY_LIST: Vec<User<Seoul>> = Vec::new();
