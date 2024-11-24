use poem_openapi::param::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

use crate::db;

#[derive(Object)]
pub struct TaskStatus {
    pub id: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

#[derive(ApiResponse)]
enum GetTaskStatusResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskStatus>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(Object)]
pub struct TaskStatusList {
    data: Vec<TaskStatus>,
    total: i64,
}

#[derive(ApiResponse)]
enum GetTaskStatusListResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskStatusList>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(Object)]
pub struct CreateTaskStatus {
    name: String
}

#[derive(ApiResponse)]
enum CreateTaskStatusResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskStatus>),
    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

pub struct TaskStatusApi {
    pub db_pool: Box<sqlx::PgPool>,
}

#[OpenApi]
impl TaskStatusApi {
    #[oai(path = "/task_status/:pk", method = "get")]
    async fn get_task_status(&self, pk: Path<i64>) -> GetTaskStatusResponse {
        let db_pool = self.db_pool.clone();
        let task_status = db::task_status::get_task_status(&db_pool, &pk.0).await;

        match task_status {
            Ok(data) => GetTaskStatusResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => GetTaskStatusResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task_status", method = "get")]
    async fn get_task_status_list(&self) -> GetTaskStatusListResponse {
        let db_pool = self.db_pool.clone();
        let task_status_list = db::task_status::get_task_status_list(&db_pool).await;

        match task_status_list {
            Ok(data) => {
                let mut list: Vec<TaskStatus> = Vec::new();
                let db_list: Vec<db::task_status::TaskStatus> = data;
                for db_task in db_list.iter() {
                    list.push(db_task.to_api());
                }
                let total: i64 = list.len() as i64;

                GetTaskStatusListResponse::SuccessResponse(Json(TaskStatusList {
                    data: list,
                    total,
                }))
            }
            Err(err) => GetTaskStatusListResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task_status", method = "post")]
    async fn create_task_status(&self, task_status: Json<CreateTaskStatus>) -> CreateTaskStatusResponse {
        let db_pool = self.db_pool.clone();
        let new_task_status = db::task_status::create_task_status(&db_pool, task_status.0.name).await;

        match new_task_status {
            Ok(data) => CreateTaskStatusResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => CreateTaskStatusResponse::Error(Json(crate::api::Error{
                msg: format!("{:?}", err)
            }))
        }
    }
}
