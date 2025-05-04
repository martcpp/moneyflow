use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow,Serialize,Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub balance: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: u64,
    pub role: String,
    pub exp: u64,
}