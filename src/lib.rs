use actix_web::http::Error;
use actix_web::{App,HttpServer};
use actix_web::dev::Server;

mod controllers;
use controllers::auth::{login,logout};
use controllers::index::{index_url,checker};

mod db;
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




pub fn run_server() -> Result<Server, Error> {
   
   let server = HttpServer::new(move|| {
        App::new()
            .service(index_url)
            .service(checker)
            .service(login)
            .service(logout)
            
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run();
    Ok(server)

    
}