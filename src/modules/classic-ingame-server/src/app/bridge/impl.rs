pub fn resolve_js_handler_resolution_keys(
    value: JsonValue,
) -> Result<(), Box<dyn std::error::Error>> {
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
