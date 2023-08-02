use serde::de::DeserializeOwned;
use std::fs;

pub fn json_string_to_struct<T: DeserializeOwned>(json_string: &str) -> T {
        serde_json::from_str::<T>(&json_string).expect("Unable to deserialize string")
}

pub fn json_file_to_struct<T: DeserializeOwned>(path: &str) -> T {
        json_string_to_struct(&(fs::read_to_string(path).expect(&format!("Unable to read path: {}", path))))
}