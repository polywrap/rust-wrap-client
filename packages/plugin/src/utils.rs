use inflector::cases::snakecase::to_snake_case;
use polywrap_msgpack::rmpv::Value;

pub fn convert_keys_to_snake_case(value: &Value) -> Value {
    match value {
        Value::Map(map) => {
            let mut new_map = map.clone();
            new_map = new_map
                .iter_mut()
                .map(|(key, value)| {
                    let new_key = to_snake_case(key.as_str().unwrap());
                    *value = convert_keys_to_snake_case(value);
                    (new_key.into(), value.clone())
                })
                .collect();
            Value::Map(new_map)
        }
        Value::Array(array) => {
            let new_array = array
                .iter()
                .map(|value| convert_keys_to_snake_case(value))
                .collect();
            Value::Array(new_array)
        }
        _ => value.clone(),
    }
}
