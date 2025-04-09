use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn index_url() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!this the home page")
}

#[get("/checker")]
pub async fn checker() -> impl Responder {
    "Hello world!this the home page".to_string()
}
