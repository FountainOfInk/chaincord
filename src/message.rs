use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub author: String,
    pub contents: String
}