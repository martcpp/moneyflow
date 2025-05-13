use crate::routers;
use actix_web::dev::Server;
use actix_web::http::Error;
use crate::middleware::auth::verify_jwt;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env::var;
use tokio::sync::Mutex;
use actix_web::middleware::from_fn;

// use actix_web::{
//     web::{JsonConfig},HttpResponse, Responder,
//     http::StatusCode,
// };

// fn json_config() -> JsonConfig {
//     web::JsonConfig::default()
//         .error_handler(|err, _req| {
//             // Custom error response
//             actix_web::error::InternalError::from_response(
//                 err,
//                 HttpResponse::BadRequest().json({
//                     serde_json::json!({
//                         "error": "Bad request. Invalid or extra fields."
//                     })
//                 }),
//             )
//             .into()
//         })
// }

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
            // .app_data(json_config())
            .app_data(state.clone())
            .service(routers::router())
            .wrap(from_fn(verify_jwt))
            // .service(routers::auth_router())
            
    })
    .bind(("localhost", 8080))
    .unwrap()
    .run();
    Ok(server)
}
