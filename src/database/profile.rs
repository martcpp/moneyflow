use crate::models::user::User;

// get all users from the database
pub async fn fetch_all_users(db: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;

    Ok(users)
}

pub async fn fetch_user_by_id(db: &sqlx::PgPool, id: &u64) -> Option<User> {
    let id_i64 = *id as i64;
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id_i64)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn fetch_user_by_email(db: &sqlx::PgPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email ILIKE $1", email)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn update_profile(
    db: &sqlx::PgPool,
    id: &u64,
    first: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    let id_i64 = *id as i64;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET first_name = $1, email = $2 WHERE id = $3 RETURNING *",
        first,
        email,
        id_i64
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}