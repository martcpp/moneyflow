use crate::models::user::User;


// get all users from the database
pub async fn fetch_all_users(db: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users"
    )
    .fetch_all(db)
    .await?;

    Ok(users)
}

pub async fn fetch_user_by_id(db: &sqlx::PgPool, id: &i64) -> Option<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        id
    )
    .fetch_optional(db)
    .await
    .unwrap();
    user
}


pub async fn fetch_user_by_email(db: &sqlx::PgPool, email: &str) -> Option<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email ILIKE $1",
        email
    )
    .fetch_optional(db)
    .await
    .unwrap();
    user
}