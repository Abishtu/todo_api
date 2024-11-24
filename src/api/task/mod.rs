use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

use crate::db;

#[derive(Object)]
pub struct Task {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(ApiResponse)]
enum GetTaskResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<Task>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(Object)]
pub struct TaskList {
    data: Vec<Task>,
    total: i64,
}

#[derive(ApiResponse)]
enum GetTaskListResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskList>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(Object)]
struct CreateTask {
    name: String,
    description: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(ApiResponse)]
enum CreateTaskResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<Task>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(Object)]
struct EditTask {
    name: Option<String>,
    description: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(ApiResponse)]
enum EditTaskResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<Task>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

pub struct TaskApi {
    pub db_pool: Box<sqlx::PgPool>,
}

#[OpenApi]
impl TaskApi {
    #[oai(path = "/task/:pk", method = "get")]
    async fn get_task(&self, pk: Path<i64>) -> GetTaskResponse {
        let db_pool = self.db_pool.clone();
        let task = db::task::get_task(&db_pool, &pk.0).await;

        match task {
            Ok(data) => GetTaskResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => GetTaskResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task/:pk", method = "put")]
    async fn edit_task(&self, pk: Path<i64>, task: Json<EditTask>) -> EditTaskResponse {
        let db_pool = self.db_pool.clone();

        let is_name = match task.0.name {
            Some(_) => true,
            None => false,
        };

        let is_description = match task.0.description {
            Some(_) => true,
            None => false,
        };

        let is_start_date = match task.0.start_date {
            Some(_) => true,
            None => false,
        };

        let is_end_date = match task.0.end_date {
            Some(_) => true,
            None => false,
        };

        let modified_task = db::task::update_task(
            &db_pool,
            pk.0,
            is_name,
            task.0.name,
            is_description,
            task.0.description,
            is_start_date,
            task.0.start_date,
            is_end_date,
            task.0.end_date,
        )
        .await;

        match modified_task {
            Ok(data) => EditTaskResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => EditTaskResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task", method = "get")]
    async fn get_task_list(&self) -> GetTaskListResponse {
        let db_pool = self.db_pool.clone();
        let task_list = db::task::get_task_list(&db_pool).await;

        match task_list {
            Ok(data) => {
                let mut list: Vec<Task> = Vec::new();
                let db_list: Vec<db::task::Task> = data;
                for db_task in db_list.iter() {
                    list.push(db_task.to_api());
                }
                let total: i64 = list.len() as i64;

                GetTaskListResponse::SuccessResponse(Json(TaskList { data: list, total }))
            }
            Err(err) => GetTaskListResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task", method = "post")]
    async fn create_task(&self, task: Json<CreateTask>) -> CreateTaskResponse {
        let db_pool = self.db_pool.clone();
        let new_task = db::task::create_task(
            &db_pool,
            task.0.name,
            task.0.description,
            task.0.start_date,
            task.0.end_date,
        )
        .await;

        match new_task {
            Ok(data) => CreateTaskResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => CreateTaskResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }
}
