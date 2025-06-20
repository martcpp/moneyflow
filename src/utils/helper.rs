use actix_web::{HttpRequest,HttpMessage};
pub fn get_auth_id(req: &HttpRequest) -> u64 {
  let ext = req.extensions();
  ext.get::<u64>().unwrap().to_owned()

}

