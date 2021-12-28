mod r#impl;

use json::JsonValue;
use std::collections::HashMap;

use self::r#impl::{receive_user_data, resolve_js_handler_resolution_keys};

use super::event::main_game::game_manager::WAITING_USERS;

pub type BridgeMapType = HashMap<String, BridgeHandlerType>;

pub type BridgeHandlerType = Box<dyn Fn(JsonValue) -> Result<(), Box<dyn std::error::Error>>>;

pub fn manager() -> BridgeMapType {
    let mut map: BridgeMapType = HashMap::new(); // TODO : Complete this

    map.insert(String::from("print"), Box::new(print));
    map.insert(
        String::from("resolve_js_handler_resolution_keys"),
        resolve_js_handler_resolution_keys,
    );
    map.insert(String::from("receive_user_data"), receive_user_data);
    map
}

pub fn resolver(event: String, data: JsonValue) -> Result<(), String> {
    let manager_data = manager();
    let v = match manager_data.get(&event) {
        Some(v) => v,
        None => return Err(String::from("invalid event name")),
    };

    match v(data) {
        Ok(v) => (),
        Err(_) => return Err(String::from("event handler has failed to resolve")),
    };

    Ok(())
}

fn print(value: JsonValue) -> Result<(), Box<dyn std::error::Error>> {
    println!("value: {}", value.to_string());
    Ok(())
}
