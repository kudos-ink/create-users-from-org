use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
pub struct CreatedUser {
    pub id: i32,
}
