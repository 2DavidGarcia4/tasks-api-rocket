use rocket::serde::json::Json;
use rocket::http::Status;

use crate::models::auth::{ResponseToken, UserCredentials};
use crate::utils::error::ErrorResponse;


#[post("/login", data = "<user_credentials>")]
pub fn login(user_credentials: Json<UserCredentials>) -> Result<Json<ResponseToken>, (Status, Json<ErrorResponse>)> {
    ResponseToken::login(user_credentials.0)
}