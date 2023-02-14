#[macro_use]
extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod db;
mod models;
mod services;
mod utils;
mod schema;

mod routes;
use routes::auth_routes::login;
use routes::user_routes::*;
use routes::task_routes::*;

pub struct CORS;

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


const ROUTE_PREFIX: &str = "/api/v1";

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS)
    .mount(ROUTE_PREFIX.to_owned()+"/auth", routes![register, login])
    .mount(ROUTE_PREFIX.to_owned()+"/users", routes![get_user, update_user, delete_user])
    .mount(ROUTE_PREFIX.to_owned()+"/tasks", routes![get_tasks, get_task, create_task, update_task, delete_task])
}
