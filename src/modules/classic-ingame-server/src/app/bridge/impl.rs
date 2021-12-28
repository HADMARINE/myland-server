use std::collections::HashMap;

use json::JsonValue;

use crate::app::{
    event::main_game::game_manager::WAITING_USERS, js_bridge::WAITING_RESOLUTION_KEYS,
};

pub fn resolve_js_handler_resolution_keys(
    value: JsonValue,
) -> Result<(), Box<dyn std::error::Error>> {
    if !value.is_object() {
        // TODO error - data invalid
    }

    let value_mapped = HashMap::new();

    for (k, v) in value.entries() {
        value_mapped.insert(k, v);
    }

    let rkey = value_mapped.get("rkey");
    let data = value_mapped.get("data");

    if rkey == None || data == None {
        // TODO error - data invalid
    }

    {
        let Some(rkey) = rkey;
        let Some(data) = data;

        WAITING_RESOLUTION_KEYS.insert(rkey.to_string(), data);
    }

    Ok(())
}

pub fn receive_user_data(value: JsonValue) -> Result<(), Box<dyn std::error::Error>> {
    let value_map = HashMap::new();
    for (k, v) in value.entries() {
        value_map.insert(k.to_string(), v);
    }

    let ids: Vec<String> = match value_map.get("data".to_string()) {
        Ok(v) => v,
        Err(e) => return e,
    };

    WAITING_USERS = ids;
}
