use rocket::serde::json::Json;
use rocket::http::Status;
use uuid::Uuid;
use crate::utils::error::ErrorResponse;
use crate::db::establish_connection;
use crate::models::tasks::{Task, TaskJson, TaskUpdateJson};

pub fn all_tasks(user_id: Uuid) -> Result<Json<Vec<Task>>, (Status, Json<ErrorResponse>)> {
    let mut connection = establish_connection();
    if let Ok(tasks) = Task::get_all(&mut connection, user_id) {
        Ok(Json(tasks))
    } else {
        Err(ErrorResponse::create_error(Status::NotFound, "Data not found.".to_owned()))
    }
}

pub fn one_task(task_id: &str, user_id: Uuid) ->  Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    let connection = &mut establish_connection();
    if let Ok(task_uuid) = Uuid::parse_str(task_id) {
        if let Ok(task) = Task::get_one(connection, user_id, task_uuid) {
        Ok(Json(task))
        } else {
        Err(ErrorResponse::create_error(Status::NotFound, "Data not found.".to_owned()))
        }
    } else {
        Err(ErrorResponse::create_error(Status::BadRequest, "Invalid token.".to_owned()))
    }
}

pub fn create_task(user_id: Uuid, task_data: TaskJson) ->  Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    let connection = &mut establish_connection();
    if let Ok(task) = Task::create(connection, &user_id, task_data) {
        Ok(Json(task))
    } else {
        Err(ErrorResponse::create_error(Status::InternalServerError, "Failed to create a new task.".to_owned()))
    }
}

pub fn update_task(task_id: &str, task_data: &TaskUpdateJson) -> Result<Json<Task>, (Status, Json<ErrorResponse>)> {
    let mut connection = establish_connection();
    if let Ok(task_uuid) = Uuid::parse_str(task_id) {
        if let Ok(task) = Task::update(&mut connection, task_uuid, task_data) {
            Ok(Json(task))
        } else {
            Err(ErrorResponse::create_error(Status::InternalServerError, "Failed to update task.".to_owned()))
        }
    } else {
        Err(ErrorResponse::create_error(Status::BadRequest, "Invalid task id.".to_owned()))
    }
}

pub fn delete_task(task_id: &str) -> (Status, Json<ErrorResponse>) {
    if let Ok(task_uuid) = Uuid::parse_str(task_id) {
        let mut connection = establish_connection();
        if let Ok(delete_result) = Task::delete(&mut connection, task_uuid) {
            ErrorResponse::create_error(
                Status::NoContent,
                format!("Task deleted: {}", delete_result)
            )
        } else {
            ErrorResponse::create_error(
                Status::BadRequest,
                "Invalid task id".to_owned()
            )
        }
    } else {
        ErrorResponse::create_error(Status::BadRequest, "Invalid task id.".to_owned())
    }
}