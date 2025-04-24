use actix_web::{HttpResponse, post, web};
use serde_json::json;
use validator::Validate;
use crate::server::AppState;
use crate::validator::authvalidator::Loginvalidation;
use crate::database::auth::{
    check_user_email,
    create_user,
};

#[post("/auth/register")]
pub async fn register(state: web::Data<AppState>, data: web::Json<Loginvalidation>) -> HttpResponse {
    // Extract the database connection from the state
    let db = state.db.lock().await;
    // Perform validation
   
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": e.to_string()
        }));
    }

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
        Ok(_) => {
            return HttpResponse::Created().json(json!({
                "status": "success",
                "message": "Account created successfully"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to create user: {}", e)
            }));
        }
    }

   }
    
   


#[post("/auth/login")]
pub async fn login() -> HttpResponse {
    // Placeholder for login logic
    // You can implement your login logic here
    HttpResponse::Ok().body("Login endpoint")
}

#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}
