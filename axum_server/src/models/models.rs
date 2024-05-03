use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ProfileRequest {
    pub username: String,
}
