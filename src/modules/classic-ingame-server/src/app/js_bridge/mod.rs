use std::collections::HashMap;

use json::{object, JsonValue};

use crate::execute_js_handler;

pub static WAITING_RESOLUTION_KEYS: HashMap<String, Option<JsonValue>> = HashMap::new();

pub fn validate_user(userid: String, token: String) -> bool {
    let rkey = uuid::Uuid::new_v4().to_string(); // Resolution key (identifier)

    execute_js_handler(
        String::from("validate_user"),
        object! {
            rkey: rkey,
            data : {
                userid: userid,
                token: token
            }
        },
    );

    WAITING_RESOLUTION_KEYS.insert(rkey, None);

    while WAITING_RESOLUTION_KEYS.get(rkey) == Some(None) {}

    let Some(Some(data)) = WAITING_RESOLUTION_KEYS.get(rkey);
    let mapped_data = HashMap::new();

    if !data.is_object() {
        // TODO : Error - invalid value
    }

    for (k, v) in data.entries() {
        mapped_data.insert(String::from(k), v);
    }

    let res = mapped_data.get(String::from("result"));
    if res == None {
        // TODO : Error - invalid value format
    }

    let Some(res) = res;

    let res = res.as_bool();

    if res == None {
        // TODO : Error - invalid value format
    }

    res
}
