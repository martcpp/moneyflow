use actix_web::{HttpResponse, get, post};

#[get("/profile")]
pub async fn get_profile() -> HttpResponse {
    HttpResponse::Ok().body("profile endpoint")
}

#[post("/profile")]
pub async fn update_profile() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}
