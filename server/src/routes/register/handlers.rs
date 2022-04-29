use axum::extract::Form;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

pub async fn create_user(Form(body): Form<CreateUser>) -> String {
    serde_json::to_string(&body).unwrap()
}
