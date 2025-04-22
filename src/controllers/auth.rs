use actix_web::{HttpResponse, post, web};
use validator::Validate;
use crate::server::AppState;
use crate::validator::authvalidator::Loginvalidation;
use crate::models::user::{
    check_user_email,
    create_user,
};

#[post("/auth/register")]
pub async fn register(state: web::Data<AppState>, data: web::Json<Loginvalidation>) -> HttpResponse {
    // Extract the database connection from the state
    let db = state.db.lock().await;
    // Perform validation
   
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().body(format!("Validation error: {:?}", e));
    }

    if check_user_email(&db, &data.email).await {
        return HttpResponse::BadRequest().body(format!("User already exists: {:?}", data.email));
    }

   
    
    // // Check if user already exists
    // let user_exists = check_user_email(&db, &data.email).await;
    // println!("User exists: {:?}", user_exists);
    // if user_exists {
    //     return HttpResponse::BadRequest().body(format!("User already exists {:?}", user_exists));
    // }
   create_user(&db, &data).await;
 return HttpResponse::Created().body(format!("User created successfully: {:?}", data));
    
       
   }
    
   


#[post("/auth/login")]
pub async fn login() -> HttpResponse {
    // Placeholder for login logic
    // You can implement your login logic here
    HttpResponse::Ok().body("Login endpoint")
}

#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}
