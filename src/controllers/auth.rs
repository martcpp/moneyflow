use actix_web::{HttpResponse, post, web};
use validator::Validate;

use crate::validator::authvalidator::Loginvalidation;

#[post("/auth/register")]
pub async fn register(data: web::Json<Loginvalidation>) -> HttpResponse {
    // Perform validation
    match data.validate() {
        Ok(_) => {
            let content = &data.into_inner();
            HttpResponse::Ok().body(format!("{:#?} ", content))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Validation error: {:?}", e)),
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
