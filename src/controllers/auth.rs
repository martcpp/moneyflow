use crate::database::auth::{check_user_email, create_user};
use crate::database::profile::fetch_user_by_email;
use crate::models::user::Claims;
use crate::server::AppState;
use crate::validator::authvalidator::{Loginvalidation, Registervalidation};
use actix_web::{HttpResponse, post, web};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use std::time::SystemTime;
use validator::Validate;

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    data: web::Json<Registervalidation>,
) -> HttpResponse {
    // Extract the database connection from the state
    let db = state.db.lock().await;

    // Perform validation
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": e.to_string()
        }));
    }

    // Check if the email is already in use
    if check_user_email(&db, &data.email).await {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "Account with this email already exists"
        }));
    }

    // // Check if user already exists
    // let user_exists = check_user_email(&db, &data.email).await;
    // println!("User exists: {:?}", user_exists);
    // if user_exists {
    //     return HttpResponse::BadRequest().body(format!("User already exists {:?}", user_exists));
    // }
    match create_user(&db, &data).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "status": "success",
            "message": "Account created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create user: {}", e)
        })),
    }
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, data: web::Json<Loginvalidation>) -> HttpResponse {
    let db = state.db.lock().await;
    // Perform validation
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": e.to_string()
        }));
    }
    let Some(user) = fetch_user_by_email(&db, &data.email).await else {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid email or password"
        }));
    };
    // let encyrupted_password = bcrypt::verify(&data.password, &user.password);
    if !bcrypt::verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid email or password"
        }));
    }

    let claim = Claims {
        sub: user.id as u64,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 2 * 60,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .unwrap();

    // You can implement your login logic here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Login successful",
        "token": token,
         // Add any other user fields you want to return
    }))
}

#[post("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}
