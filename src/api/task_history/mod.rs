use poem_openapi::param::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

#[derive(Object)]
pub struct TaskHistory {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub task_id: i64,
    pub status_id: i64,
}

#[derive(Object)]
pub struct TaskHistoryList {
    data: Vec<TaskHistory>,
    total: i64,
}

#[derive(ApiResponse)]
enum GetTaskHistoryResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskHistory>),

    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

#[derive(ApiResponse)]
enum GetTaskHistoryListResponse {
    #[oai(status = 200)]
    SuccessResponse(Json<TaskHistoryList>),

    #[oai(status = 500)]
    Error(Json<crate::api::Error>),
}

pub struct TaskHistoryApi {
    pub db_pool: Box<sqlx::PgPool>,
}

#[OpenApi]
impl TaskHistoryApi {
    #[oai(path = "/task_history/:pk", method = "get")]
    async fn get_task_history(&self, pk: Path<i64>) -> GetTaskHistoryResponse {
        let db_pool = self.db_pool.clone();
        let task_history = crate::db::task_history::get_task_history(&db_pool, &pk.0).await;

        match task_history {
            Ok(data) => GetTaskHistoryResponse::SuccessResponse(Json(data.to_api())),
            Err(err) => GetTaskHistoryResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }

    #[oai(path = "/task_history", method = "get")]
    async fn get_task_history_list(&self) -> GetTaskHistoryListResponse {
        let db_pool = self.db_pool.clone();
        let task_history_list = crate::db::task_history::get_task_history_list(&db_pool).await;

        match task_history_list {
            Ok(data) => {
                let mut list: Vec<TaskHistory> = Vec::new();
                let db_list: Vec<crate::db::task_history::TaskHistory> = data;
                for db_task in db_list.iter() {
                    list.push(db_task.to_api());
                }
                let total: i64 = list.len() as i64;

                GetTaskHistoryListResponse::SuccessResponse(Json(TaskHistoryList {
                    data: list,
                    total,
                }))
            }
            Err(err) => GetTaskHistoryListResponse::Error(Json(crate::api::Error {
                msg: format!("{:?}", err),
            })),
        }
    }
}
