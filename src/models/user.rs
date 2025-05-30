use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id:Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}
