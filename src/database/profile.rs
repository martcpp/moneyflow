use crate::models::user::User;

pub async fn fetch_all_users(db: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, first_name, last_name, email, password, balance, created_at, updated_at FROM users"
    )
    .fetch_all(db)
    .await?;

    Ok(users)
}
