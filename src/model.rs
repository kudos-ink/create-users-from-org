use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
}
