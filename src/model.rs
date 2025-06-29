use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
    pub avatar: String,
    pub email: Option<String>,
    pub github_id: i64
}

#[derive(Deserialize, Debug)]
pub struct CreatedUser {
    pub id: i32,
}
