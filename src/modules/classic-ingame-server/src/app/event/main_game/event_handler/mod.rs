use std::collections::HashMap;

use json::JsonValue;

use crate::{
    error::predeclared::QuickSocketError,
    socket_instance::{ChannelClient, ChannelController},
};

pub fn register(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {
    let u_uuid = (ctrl.accepted_client as ChannelClient).uid;

    if !ctrl.value.is_object() {
        return Err(QuickSocketError::JsonParseFail.to_box());
        // TODO : Error handle
    }

    let mapped_data = HashMap::new();

    for (k, v) in ctrl.value.entries() {
        mapped_data.insert(k, v);
    }

    let userid = match mapped_data.get("userid") {
        Some(v) => v.to_string(),
        None => {
            return Err(QuickSocketError::JsonFormatInvalid.to_box());
        }
    };
    let token = match mapped_data.get("token") {
        Some(v) => v.to_string(),
        None => {
            return Err(QuickSocketError::JsonFormatInvalid.to_box());
        }
    };

    ctrl.channel.;
}

pub fn user_ready(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {
    (ctrl.accepted_client as ChannelClient).uid;
}

// pub fn user_ready(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {}
