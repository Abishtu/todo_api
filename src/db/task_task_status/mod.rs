use sqlx::{Decode, FromRow, Type};
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Type, Serialize, Deserialize)]
pub struct TaskTaskStatus {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub task_id: i64,
    pub task_status_id: i64,
}

pub async fn get_tasks_task_status_list(
    conn: &sqlx::PgPool,
    is_task_id: bool,
    task_id: Option<Vec<i64>>,
) -> Result<Vec<TaskTaskStatus>, Box<dyn Error>> {
    let q = r"
        SELECT * FROM tasks_task_status
        CASE
            WHEN $1 THEN task_id = ANY($2) ELSE true
        END
    ";
    let query = sqlx::query_as::<_, TaskTaskStatus>(q)
        .bind(is_task_id)
        .bind(task_id);

    let tasks_task_status_list = query.fetch_all(conn).await?;

    Ok(tasks_task_status_list)
}

pub async fn create_tasks_task_status(
    conn: &sqlx::PgPool,
    task_id: i64,
    task_status_id: i64,
) -> Result<TaskTaskStatus, Box<dyn Error>> {
    let q = r"
        INSERT INTO tasks_task_status (
            task_id,
            task_status_id
        ) VALUES ($1, $2)
        RETURNING *
    ";
    let query = sqlx::query_as::<_, TaskTaskStatus>(q)
        .bind(task_id)
        .bind(task_status_id);

    let new_task_task_status = query.fetch_one(conn).await?;

    Ok(new_task_task_status)
}

pub async fn delete_task_task_status_association(
    conn: &sqlx::PgPool,
    task_id: i64,
    task_status_id: i64,
) -> Result<TaskTaskStatus, Box<dyn Error>> {
    let q = r"
        DELETE FROM tasks_task_stauts
        WHERE task_id = $1
        AND task_status_id = $2
        RETURNING *
    ";
    let query = sqlx::query_as::<_, TaskTaskStatus>(q)
        .bind(task_id)
        .bind(task_status_id);

    let deleted_task_task_status_association = query.fetch_one(conn).await?;

    Ok(deleted_task_task_status_association)
}
