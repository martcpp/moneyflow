use actix_web::{get, post, web, HttpRequest, HttpResponse};
use crate::server::AppState;
use crate::database::profile::{fetch_all_users,fetch_user_by_id,fetch_user_by_email};
use serde_json::json;

// Get all users
#[get("/profile")]
pub async fn get_profile(state:web::Data<AppState>) -> HttpResponse {
    let db = state.db.lock().await;
    let users = fetch_all_users(&db).await;
    match users {
        Ok(users) => {
            return HttpResponse::Ok().json(
                json!({
                    "status": "success",
                    "data": users
                })
            );
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
   
// Get a user profile by ID
#[get("/profile/{id}")]
/// Get a user profile by email
pub async fn get_profile_by_id(state:web::Data<AppState>, path: web::Path<i64>) -> HttpResponse {
    let db = state.db.lock().await;
    let id= path.into_inner();
    //println!("User ID: {:?}", id);

    let user = fetch_user_by_id (&db,&id).await;
    println!("User: {:?}", user);
    match user {
        Some(user_d) => {
            return HttpResponse::Ok().json(
                json!({
                    "status": "success",
                    "data": user_d
                })
            );
        }
        None => {
            return HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "User not found"
            }));
        }
    };

}
#[get("/profile/update")]
/// Update user profile
pub async fn update_profile(req:HttpRequest) -> HttpResponse {
    // Extract the user ID from the request
    let user_id = req.match_info();
    println!("User ID: {:?}", user_id);
    HttpResponse::Ok().body(format!("User ID: {:?}", user_id))
}
