use crate::routers::router;
use actix_web::dev::Server;
use actix_web::http::Error;
use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;
use crate::configuration::get_config;


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
    // Load environment variables from .env file
    let config = get_config().expect("Failed to load configuration");
    let database_url = config.database_url();

    // Initialize the database connection pool
    let state = web::Data::new(AppState {
        db: Mutex::new(
            sqlx::PgPool::connect(&database_url)
                .await
                .unwrap(),
        ),
        jwt_secret: config.jwt_secret().to_string(),
    });

    let server = HttpServer::new(move || {
        App::new()
            // .app_data(json_config())
            .app_data(state.clone())
            .configure(router)
    
            // .service(routers::auth_router())
            
    })
    .bind((config.server_host(), config.server_port()))
    .unwrap()
    .run();
    Ok(server)
}
