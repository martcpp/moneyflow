use crate::validator::authvalidator::Loginvalidation;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::types::chrono;

// pub async fn check_user_email(db:&sqlx::PgPool,email:&str)-> bool{
//     sqlx::query!(r#"
//         SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
//     "#, email)
//     .fetch_optional (db)
//     .await
//     .unwrap()
//     .is_some()

// }

pub async fn check_user_email(db: &sqlx::PgPool, email: &str) -> bool {
    sqlx::query!("
        SELECT * FROM users WHERE email ILIKE $1
    ", email)
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()
    

}



// struct User{
//     id:u64,
//     email:String,
//     first_name:String,
//     last_name:String,
//     password:String,
//     balance:u32,
//     created_at:chrono::NaiveDateTime,
//     updated_at:chrono::NaiveDateTime,
// }

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