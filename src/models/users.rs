use diesel::prelude::*;
use uuid::Uuid;
use diesel::pg::PgConnection;
use rocket::serde::{Serialize, Deserialize};
use diesel::result::Error;
use chrono::{DateTime, Utc};

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::schema::tasks;
use crate::models::tasks::TaskUser;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub image_url: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub image_url: Option<&'a String>
} 

#[derive(Debug, Deserialize)]
pub struct UserJson<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserTasks {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub image_url: Option<String>,
    pub tasks: Vec<TaskUser>
}


#[derive(Debug, Serialize)]
pub struct UserAndToken {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub image_url: Option<String>,
    pub token: String
} 

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdateJson<'a> {
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub image_url: Option<&'a str>,
}

impl User {
    pub fn get_all(conn: &mut PgConnection) -> Vec<User> {
        users.load(conn).expect("Error get all users")
    }

    pub fn get_by_id(conn: &mut PgConnection, user_id: &Uuid) -> Option<UserTasks> {
        let results: Result<Vec<(User, Uuid, String, String, DateTime<Utc>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, bool)>, Error> = tasks::table
            .inner_join(users::table)
            .select((users::all_columns, tasks::id, tasks::title, tasks::description, tasks::created_at, tasks::notification_at, tasks::completed_at, tasks::is_completed))
            .load::<(User, Uuid, String, String, DateTime<Utc>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, bool)>(conn);


        if let Ok(result_users) = results{
            let mut users_tasks: Vec<UserTasks> = vec![];

            for (user, task_id, title, description, created_at, notification_at, completed_at, is_completed) in result_users {
                let mut user_tasks = UserTasks {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    password: user.password,
                    image_url: user.image_url,
                    tasks: vec![],
                };

                let task_user = TaskUser { id: task_id, title, description, created_at, notification_at, completed_at, is_completed };
                if let Some(ut) = users_tasks.iter_mut().find(|ut| ut.id == user.id) {
                    ut.tasks.push(task_user);
                } else {
                    user_tasks.tasks.push(task_user);
                    users_tasks.push(user_tasks);
                }
            }
            let user = users_tasks.into_iter().find(|u| &u.id == user_id);
            user

        } else {
            None
        }
    }

    pub fn create_user(conn: &mut PgConnection, new_name: &str, new_email: &str, new_password: &str) -> Result<User, Error> {
        let new_user = NewUser {
            id: &Uuid::new_v4(), 
            name: new_name, 
            email: new_email, 
            password: new_password,
            image_url: None
        };
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
    }

    pub fn update(conn: &mut PgConnection, user_id: Uuid, data: UserUpdateJson) -> Result<User, Error> {
        diesel::update(users::table.find(user_id))
        .set(data)
        .get_result::<User>(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize>  {
        diesel::delete(users.find(user_id)).execute(conn)
    }
}