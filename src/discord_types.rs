use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub author: User,
    pub content: String
}

#[derive(Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String
}


#[derive(Deserialize)]
pub struct ApiError {
    pub code: u32,
    // TODO: errors field
    pub message: String 
}

// TODO: actually implement snowflake
type Snowflake = String;