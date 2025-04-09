use crate::controllers::{auth,profile,index};
use auth::{login,logout};
use profile::{get_profile,update_profile};
use index::{index_url,checker};


pub fn router() -> actix_web::Scope {
    actix_web::web::scope("/api/v1")
        .service(index_url)
        .service(checker)
        .service(login)
        .service(logout)
        .service(get_profile)
        .service(update_profile)
    
}