use crate::routers;
use actix_web::dev::Server;
use actix_web::http::Error;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env::var;
use tokio::sync::Mutex;
pub struct AppState {
    // Add any shared state here
    pub db: Mutex<sqlx::PgPool>,
    pub jwt_secret: String,
}
// use routers::router

pub async fn run_server() -> Result<Server, Error> {
    dotenv().ok();
    // Load environment variables from .env file

    // Initialize the database connection pool
    let state = web::Data::new(AppState {
        db: Mutex::new(
            sqlx::PgPool::connect(&var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        ),
        jwt_secret: var("JWT_SECRET").unwrap_or("test123".to_string()),
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
