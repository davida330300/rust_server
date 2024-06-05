use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorRegisterForm {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}