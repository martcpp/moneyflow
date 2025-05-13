use crate::controllers::{auth, index, profile};
use auth::{login, logout, register};
use index::{checker, index_url};
use profile::{get_profile, get_profile_by_id, update_profile};
use actix_web::services;

pub fn router() -> actix_web::Scope{
        actix_web::web::scope("/api/v1/auth")
        .service(index_url)
        .service(register)
        //actix_web::web::scope("/api/v2")
        .service(login)
        .service(checker)
        //.service(register)
        //actix_web::web::scope("/api/v2")
        //.service(login)
        .service(logout)
        .service(get_profile)
        .service(update_profile)
        .service(get_profile_by_id)
        
}

// pub fn auth_router() ->  {
   
// }
