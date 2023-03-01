use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}