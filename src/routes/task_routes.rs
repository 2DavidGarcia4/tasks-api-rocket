use rocket::http::Status;
use rocket::serde::json::Json;
use crate::models::tasks::{Task, TaskJson, TaskUpdateJson};
use crate::utils::jwt::UserToken;
use crate::utils::error::ErrorResponse;
use crate::services::task_services;

#[get("/")]
pub fn get_tasks(token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<Vec<Task>>, (Status, Json<ErrorResponse>)> {
    if let Ok(claims) = token {
        task_services::all_tasks(claims.id)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}

#[get("/<task_id>")]
pub fn get_task(task_id: &str, token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    if let Ok(claims) = token {
        task_services::one_task(task_id, claims.id)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}

#[post("/", data = "<task_data>")]
pub fn create_task(task_data: Json<TaskJson>, token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    if let Ok(claims) = token {
        task_services::create_task(claims.id, task_data.0)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}


#[put("/<task_id>", data = "<task_data>")]
pub fn update_task(task_id: &str, task_data: Json<TaskUpdateJson>, token: Result<UserToken, Json<ErrorResponse>>) -> Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    if let Ok(_) = token {
        task_services::update_task(task_id, &task_data.0)
    } else {
        Err(ErrorResponse::create_error(Status::Unauthorized, "No authorization, invalid token or not obtained.".to_owned()))
    }
}

#[delete("/<task_id>")]
pub fn delete_task(task_id: &str) -> (Status, Json<ErrorResponse>) {
    task_services::delete_task(task_id)
}