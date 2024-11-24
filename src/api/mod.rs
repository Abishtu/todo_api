use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

pub mod task;
pub mod task_history;
pub mod task_status;

#[derive(Object)]
pub struct Error {
    pub msg: String,
}
