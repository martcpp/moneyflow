use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct Loginvalidation {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
