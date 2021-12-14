use crate::{error::predeclared::QuickSocketError, socket_instance::ChannelController};

use json::JsonValue;
use std::collections::HashMap;

pub mod echo;
pub mod game_manager;
pub mod main_game;

pub type EventMapType = HashMap<String, EventHandlerType>;

pub type EventHandlerType =
    fn(ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>>;

pub fn manager(preset: &Option<String>) -> EventMapType {
    let none = String::from("none");
    let preset = match preset {
        Some(v) => v,
        None => &none,
    };
    return match preset.as_str() {
        "none" => HashMap::new(),
        "echo" => echo::get(),
        "mainGame" => main_game::get_config(),
        _ => {
            panic!("Invalid preset : {}", preset);
        }
    };
}
