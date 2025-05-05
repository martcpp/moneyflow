use crate::models::user::Claims;
use crate::server::AppState;
use actix_web::middleware::Next;
use actix_web::{
    Error, HttpMessage,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    web,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::json;
pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Missing Authorization header"
        }))
    })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid Authorization header format"
        }))
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid Authorization header format"
        })));
    }

    let token = auth_str.strip_suffix("Bearer ").unwrap();
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);

            next.call(req).await
        }
        Err(_) => Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid token"
        }))),
    }
}
