// Project: rust-webapp

pub mod controllers;
pub mod database;
pub mod dbcon;
pub mod middleware;
pub mod models;
pub mod routers;
pub mod server;
pub mod validator;
pub mod utils;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
