use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
//#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Registervalidation {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
//#[serde(rename_all = "camelCase")]
pub struct Loginvalidation {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}
