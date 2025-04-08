use actix_web::{get, HttpResponse, Responder};


#[get("/")]
pub async fn index_url() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!this the home page")
}

#[get("/checker")]
pub async fn checker() -> impl Responder {
 "Hello world!this the home page".to_string()
}