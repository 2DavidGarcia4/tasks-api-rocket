use rocket::http::Status;
use rocket::serde::json::Json;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::db::establish_connection;
use crate::models::users::{User, UserTasks, UserAndToken, UserUpdateJson};
use crate::utils::jwt::{encode_token, UserToken};
use crate::utils::error::ErrorResponse;
use crate::utils::crypt::encrypt_password;

pub fn get_by_id(user_id: &Uuid) -> Result<Json<UserTasks>, (Status, Json<ErrorResponse>)> {
    let mut connection = establish_connection();
    let pre_user = User::get_by_id(&mut connection, &user_id);
    if let Some(user) = pre_user {
        Ok(Json(user))
    } else {
        Err(ErrorResponse::create_error(Status::NotFound, "User not found".to_owned()))
    }
}

pub fn create_user(new_name: &str, new_email: &str, new_password: &str) -> Result<Json<UserAndToken>, (Status, Json<ErrorResponse>)> {
    let mut connection = establish_connection();
    let hashed_password = encrypt_password(new_password).unwrap();
    let new_user = User::create_user(&mut connection, new_name, new_email, &hashed_password.as_str());
    
    if let Ok(user) = new_user {
        let claims = UserToken {
            id: user.id,
            email: user.email.clone(),
            exp: (Utc::now() + Duration::minutes(10)).timestamp()
        };
        let token = encode_token(claims);
        Ok(
            Json(UserAndToken { 
                id: user.id, 
                name: user.name, 
                email: user.email.clone(), 
                password: user.password, 
                image_url: user.image_url, 
                token
            })
        )

    } else {
        Err((
            Status::NoContent, 
            Json(ErrorResponse { 
                message: "All fields must be completed".to_owned() 
            })
        ))
    }
}

pub fn user_update(user_id: Uuid, user_data: UserUpdateJson) -> Result<Json<User>, (Status, Json<ErrorResponse>)> {
    let mut connection = establish_connection();
    if user_data.password.is_none() {
        if let Ok(user) = User::update(&mut connection, user_id, user_data) {
            Ok(Json(user))
        } else {
            Err(ErrorResponse::create_error(Status::InternalServerError, "Failed to update user.".to_owned()))
        }
    } else {
        if let Ok(hashed_pass) = encrypt_password(user_data.password.unwrap()) {
            if let Ok(user) = User::update(&mut connection, user_id, UserUpdateJson { 
                name: user_data.name, 
                email: user_data.email, 
                password: Some(hashed_pass.as_str()), 
                image_url: user_data.image_url 
            }) {
                Ok(Json(user))
            } else {
                Err(ErrorResponse::create_error(Status::InternalServerError, "Failed to update user.".to_owned()))
            }
        } else {
            Err(ErrorResponse::create_error(Status::InternalServerError, "Failed encrypted password.".to_owned()))
        }
    }
}

pub fn delete_user(user_id: Uuid) -> (Status, Json<ErrorResponse>) {
    let mut connection = establish_connection();
    if let Ok(delete_result) = User::delete_user(&mut connection, user_id) {
        (
            Status::NoContent,
            Json(ErrorResponse {
                message: format!("User deleted: {}", delete_result)
            })
        )
    } else {
        (
            Status::BadRequest,
            Json(ErrorResponse {
                message: "Invalid user id".to_owned()
            })
        )
    }
}