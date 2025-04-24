use actix_web::{HttpResponse, get, post, web};
use crate::server::AppState;
use crate::database::profile::fetch_all_users;
use serde_json::json;


#[get("/profile")]
pub async fn get_profile(state:web::Data<AppState>) -> HttpResponse {
    let db = state.db.lock().await;
    let users = fetch_all_users(&db).await;
    match users {
        Ok(users) => {
            return HttpResponse::Ok().json(users.iter()
            .map(|user| {
                // Convert the User struct to a JSON-compatible format
                json!({
                    "status": "success",
                    "message": "Users fetched successfully",
                    "user": {
                    "id": user.id,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "email": user.email,
                    "balance": user.balance,
                    "created_at": user.created_at,
                    "updated_at": user.updated_at
                }})
            }).collect::<Vec<_>>());
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to fetch users: {}", e)
            }));
        }
    }
            // return HttpResponse::Ok().body(format!("Users: {:?}", users));
        }
   

#[post("/profile")]
pub async fn update_profile() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}
