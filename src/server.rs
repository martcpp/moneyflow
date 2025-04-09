use crate::routers;
use actix_web::dev::Server;
use actix_web::http::Error;
use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;
struct AppState {
    // Add any shared state here
    db: Mutex<sqlx::PgPool>,
}
// use routers::router

pub async fn run_server() -> Result<Server, Error> {
    // Initialize the database connection pool
    let state = web::Data::new(AppState {
        db: Mutex::new(sqlx::PgPool::connect("").await.unwrap()),
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(routers::router())
    })
    .bind(("localhost", 8080))
    .unwrap()
    .run();
    Ok(server)
}
