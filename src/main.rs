#[macro_use]
extern crate rocket;

mod db;
mod models;
mod services;
mod utils;
mod schema;

mod routes;
use routes::auth_routes::login;
use routes::user_routes::*;
use routes::task_routes::*;


const ROUTE_PREFIX: &str = "/api/v1";

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount(ROUTE_PREFIX.to_owned()+"/auth", routes![register, login])
    .mount(ROUTE_PREFIX.to_owned()+"/users", routes![get_user, update_user, delete_user])
    .mount(ROUTE_PREFIX.to_owned()+"/tasks", routes![get_tasks, get_task, create_task, update_task, delete_task])
}
