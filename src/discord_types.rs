use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub author: User,
    pub content: String
}

#[derive(Deserialize)]
pub struct User {
    pub id: String,             // actually a snowflake, might implement later
    pub username: String
}