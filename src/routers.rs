use crate::controllers::{auth, index, profile};
use auth::{login, logout, register};
use index::{checker, index_url};
use crate::middleware::auth::verify_jwt;
use profile::{get_profile, get_profile_by_id, update_profile};
use actix_web::{web::{self,ServiceConfig},middleware::from_fn};



pub fn router(cfg:&mut ServiceConfig ){
        cfg.service(
                web::scope("/api/v1")
                 //.wrap(from_fn(verify_jwt))
                 
                .service(index_url)
                .service(checker)
                
                .service(
                web::scope("/auth")
                .service(register)
                .service(login)
                .service(logout)
                )
                
                .service(
                        web::scope("")
                        .wrap(from_fn(verify_jwt))
                        .service(get_profile)
                        .service(get_profile_by_id)
                        .service(update_profile)
                        )


        );
}

// pub fn auth_router() ->  {
   
// }
