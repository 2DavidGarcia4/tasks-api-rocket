use rocket::http::Status;
use rocket::serde::json::Json;
use crate::services::user_services;
use crate::models::users::{User, UserJson, UserAndToken, UserTasks, UserUpdateJson};
use crate::utils::error::ErrorResponse;
use crate::utils::jwt::UserToken;

#[get("/me")]
pub fn get_user(token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<UserTasks>, (Status, Json<ErrorResponse>)> {
    if let Ok(claims) = token {
        user_services::get_by_id(&claims.id)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}

#[post("/register", data = "<user_data>")]
pub fn register(user_data: Json<UserJson>) -> Result<Json<UserAndToken>, (Status, Json<ErrorResponse>)> {
    user_services::create_user(user_data.name, user_data.email, user_data.password)
}

#[put("/me", data = "<user_data>")]
pub fn update_user(user_data: Json<UserUpdateJson>, token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<User>, (Status, Json<ErrorResponse>)> {
    if let Ok(claims) = token {
        user_services::user_update(claims.id, user_data.0)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}

#[delete("/me")]
pub fn delete_user(token: Result<UserToken, Json<ErrorResponse>>) -> (Status, Json<ErrorResponse>) {
    if let Ok(claims) = token {
        user_services::delete_user(claims.id)
    } else {
        ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned())
    }
}