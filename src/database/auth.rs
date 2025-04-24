use crate::validator::authvalidator::Loginvalidation;
use bcrypt::{DEFAULT_COST, hash};

pub async fn check_user_email(db: &sqlx::PgPool, email: &str) -> bool {
    sqlx::query!("
        SELECT * FROM users WHERE email ILIKE $1
    ", email)
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()


}



pub async fn create_user(db:&sqlx::PgPool, data:&Loginvalidation)->Result<(), sqlx::Error> {
    let hashed_password = hash(&data.password, DEFAULT_COST).unwrap();
   let query = sqlx::query!(
        "
        INSERT INTO users (email, first_name, last_name, password)
        VALUES ($1, $2, $3, $4)
        ",
        data.email,
        data.first_name,
        data.last_name,
        hashed_password,
    
    ).execute(db)
    .await;

  
    match query {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
    

}