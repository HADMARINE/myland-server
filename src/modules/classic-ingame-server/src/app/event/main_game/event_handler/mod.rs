use json::JsonValue;

use crate::{
    error::predeclared::QuickSocketError,
    socket_instance::{ChannelClient, ChannelController},
};

pub fn register(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {
    (ctrl.accepted_client as ChannelClient).uid;
}

pub fn user_ready(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {
    (ctrl.accepted_client as ChannelClient).uid;
}

// pub fn user_ready(ctrl: ChannelController) -> Result<Option<JsonValue>, Box<QuickSocketError>> {}
