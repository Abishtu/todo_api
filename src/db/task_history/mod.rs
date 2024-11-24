use crate::db;
use sqlx::FromRow;
use std::error::Error;

#[derive(Debug, FromRow)]
pub struct TaskHistory {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub task_id: i64,
    pub status_id: i64,
}

impl TaskHistory {
    pub fn to_api(&self) -> crate::api::task_history::TaskHistory {
        crate::api::task_history::TaskHistory {
            id: self.id,
            created_at: self.created_at,
            status_id: self.status_id,
            task_id: self.task_id,
        }
    }
}

pub async fn get_task_history_list(
    conn: &sqlx::PgPool,
) -> Result<Vec<TaskHistory>, Box<dyn Error>> {
    let q = "SELECT * FROM task_history";
    let query = sqlx::query_as::<_, TaskHistory>(q);

    let task_history_list = query.fetch_all(conn).await?;

    Ok(task_history_list)
}

pub async fn get_task_history(
    conn: &sqlx::PgPool,
    pk: &i64,
) -> Result<TaskHistory, Box<dyn Error>> {
    let q = "SELECT * FROM task_history WHERE id = $1";
    let query = sqlx::query_as::<_, TaskHistory>(q).bind(&pk);

    let result = query.fetch_one(conn).await?;

    Ok(result)
}

pub async fn create_new_task_history(
    conn: &sqlx::PgPool,
    status_id: i64,
    task_id: i64,
) -> Result<TaskHistory, Box<dyn Error>> {
    let q = r"
        INSERT INTO task_history (status_id, task_id)
        VALUES ($1, $2) RETURNING *
    ";
    let query = sqlx::query_as::<_, TaskHistory>(q)
        .bind(status_id)
        .bind(task_id);

    let result = query.fetch_one(conn).await?;

    Ok(result)
}
