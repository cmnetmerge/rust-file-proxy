use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub content: T,
}