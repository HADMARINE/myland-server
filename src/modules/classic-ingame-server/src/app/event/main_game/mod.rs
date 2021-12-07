use std::collections::HashMap;

use super::EventMapType;

pub mod constants;
pub mod cyclic_event_queue;
pub mod main_loop;
pub mod structs;
pub mod transformers;

pub fn get_config() -> EventMapType {
    let mut map: EventMapType = HashMap::new();

    map
}
