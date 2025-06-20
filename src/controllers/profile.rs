use crate::database::profile::{fetch_all_users, fetch_user_by_id};
use crate::server::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::utils::helper::get_auth_id;

// Get all users
#[get("/profile_all")]
pub async fn get_profile(state: web::Data<AppState>) -> HttpResponse {
    let db = state.db.lock().await;
    let users = fetch_all_users(&db).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": users
        })),

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch users: {}", e)
        })),
    }
    // return HttpResponse::Ok().body(format!("Users: {:?}", users));
}

// Get a user profile by ID
#[get("/profile")]
/// Get a user profile by email
pub async fn get_profile_by_id(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let db = state.db.lock().await;
    let id = get_auth_id(&req);

    //println!("User ID: {:?}", id);

    let user = fetch_user_by_id(&db, &id).await;
    println!("User: {:?}", user);
    match user {
        Some(user_d) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": user_d
        })),
        None => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User not found"
        })),
    }
}
#[get("/profile/update")]
/// Update user profile
pub async fn update_profile() -> HttpResponse {
    // Extract the user ID from the request

    HttpResponse::Ok().body(format!("User ID: {:?}", 1))
}
