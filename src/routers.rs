use crate::controllers::{auth, index, profile};
use auth::{login, logout, register};
use index::{checker, index_url};
use profile::{get_profile, update_profile};

pub fn router() -> actix_web::Scope {
    actix_web::web::scope("/api/v1")
        .service(index_url)
        .service(checker)
        .service(register)
        .service(login)
        .service(logout)
        .service(get_profile)
        .service(update_profile)
}
