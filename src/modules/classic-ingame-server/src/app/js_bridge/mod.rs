use std::collections::HashMap;

use json::{object, JsonValue};

use crate::execute_js_handler;

pub static WAITING_RESOLUTION_KEYS: HashMap<String, Option<JsonValue>> = HashMap::new();

pub fn validate_user(userid: String, token: String) -> bool {
    let rkey = uuid::Uuid::new_v4().to_string(); // Resolution key (identifier)

    execute_js_handler(String::from("validate_user"), object! {})
}
