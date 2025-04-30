use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;


#[derive(Debug, FromRow,serde::Serialize, serde::Deserialize)]
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
