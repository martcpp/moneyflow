use actix_web::http::Error;
use actix_web::{App,HttpServer};
use actix_web::dev::Server;

// use routers::router
use crate::routers;
pub fn run_server() -> Result<Server, Error> {
   
    let server = HttpServer::new(move|| {
         App::new()
            .service(routers::router())
     })
     .bind(("localhost", 8080)).unwrap()
     .run();
     Ok(server)
 
     
 }