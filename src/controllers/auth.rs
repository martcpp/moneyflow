use actix_web::{post,HttpResponse};

#[post("/auth/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login endpoint")
}


#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}