use serde::de::DeserializeOwned;
use std::fs;

pub fn json_string_to_struct<T: DeserializeOwned>(json_string: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str::<T>(&json_string)
}

pub fn json_file_to_struct<T: DeserializeOwned>(path: &str) -> Result<T, serde_json::Error> {
        json_string_to_struct::<T>(&(fs::read_to_string(path).expect(&format!("Unable to read path: {}", path))))
}