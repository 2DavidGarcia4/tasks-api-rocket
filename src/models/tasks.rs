use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use rocket::serde::{Serialize, Deserialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::schema::tasks;
use crate::schema::tasks::dsl::*;


#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub notification_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: bool,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct TaskUser {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub notification_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: bool,
}


#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub created_at: &'a DateTime<Utc>,
    pub notification_at: Option<&'a DateTime<Utc>>,
    pub completed_at: Option<&'a DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct TaskJson<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct TaskUpdate<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub notification_at: Option<&'a DateTime<Utc>>,
    pub completed_at: Option<&'a DateTime<Utc>>,
    pub is_completed: Option<&'a bool>
}

#[derive(Debug, Deserialize)]
pub struct TaskUpdateJson {
    pub title: Option<String>,
    pub description: Option<String>,
    pub notification_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: Option<bool>
}


impl Task {
    pub fn get_all(conn: &mut PgConnection, user_uuid: Uuid) -> Result<Vec<Task>, Error> {
        tasks::table.filter(tasks::dsl::user_id.eq(user_uuid)).load::<Task>(conn)
    }

    pub fn get_one(conn: &mut PgConnection, user_uuid: Uuid, task_id: Uuid) -> Result<Task, Error> {
        tasks::table.filter(tasks::dsl::user_id.eq(user_uuid)).find(task_id).first::<Task>(conn)
    }

    pub fn create(conn: &mut PgConnection, user_uuid: &Uuid, new_data: TaskJson) -> Result<Task, Error> {
        let new_date = Utc::now();
        let new_utc = Utc::now().naive_utc();
        print!("{} || {}", new_date, new_utc);

        let new_task = NewTask {
            id: &Uuid::new_v4(), 
            user_id: user_uuid, 
            title: new_data.title, 
            description: new_data.description,
            created_at: &new_date, 
            notification_at: None,
            completed_at: None
        };
        diesel::insert_into(tasks::table)
            .values(new_task)
            .get_result::<Task>(conn)
    }

    pub fn update(conn: &mut PgConnection, task_id: Uuid, data: &TaskUpdateJson) -> Result<Task, Error> {
        diesel::update(tasks::table.find(task_id))
        .set(TaskUpdate {
            title: data.title.as_ref().map(|m| m.as_str()),
            description: data.description.as_ref().map(|m| m.as_str()),
            notification_at: data.notification_at.as_ref(),
            completed_at: data.completed_at.as_ref(),
            is_completed: data.is_completed.as_ref()
        })
        .get_result::<Task>(conn)
    }

    pub fn delete(conn: &mut PgConnection, task_id: Uuid) -> QueryResult<usize>  {
        diesel::delete(tasks.find(task_id)).execute(conn)
    }
}