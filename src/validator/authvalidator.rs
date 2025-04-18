use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct Loginvalidation {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
