use actix_web::{post, web, HttpResponse};

use crate::validator::authvalidator::Loginvalidation;
#[post("/auth/login")]
pub async fn login(data:web::Json<Loginvalidation>) -> HttpResponse {
    let content = &data.into_inner();
  
    HttpResponse::Ok().body(format!("{:#?} ", content) )
}


#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}