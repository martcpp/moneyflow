use actix_web::{HttpResponse, post, web};
use validator::Validate;

use crate::validator::authvalidator::Loginvalidation;

#[post("/auth/login")]
pub async fn login(data: web::Json<Loginvalidation>) -> HttpResponse {
    match data.validate() {
        Ok(_) => {
            let content = &data.into_inner();
            HttpResponse::Ok().body(format!("{:#?} ", content))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Validation error: {:?}", e)),
    }
}

#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logout endpoint")
}

// #[post("/auth/login")]
// pub async fn login(data: web::Json<Loginvalidation>) -> Result<HttpResponse, Error> {
//     // Perform validation
//     if let Err(e) = data.validate() {
//         return Ok(HttpResponse::BadRequest().body(format!("Validation error: {:?}", e)));
//     }

//     // Extract and move the data
//     let content = data.into_inner();

//     // Return success
//     Ok(HttpResponse::Ok().body(format!("Validated login: {:#?}", content)))
// }
