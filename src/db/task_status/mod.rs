use sqlx::FromRow;
use std::error::Error;

pub enum StartingTaskStatusEntries{
    Created = 1,
    Deleted = 2,
    Open = 3,
    Closed = 4
}

#[derive(Debug, FromRow)]
pub struct TaskStatus {
    pub id: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl TaskStatus {
    pub fn to_api(&self) -> crate::api::task_status::TaskStatus {
        crate::api::task_status::TaskStatus {
            id: self.id,
            updated_at: self.updated_at,
            created_at: self.created_at,
            name: String::from(&self.name),
        }
    }
}

pub async fn get_task_status_list(conn: &sqlx::PgPool) -> Result<Vec<TaskStatus>, Box<dyn Error>> {
    let q = "SELECT * FROM task_status";
    let query = sqlx::query_as::<_, TaskStatus>(q);

    let status_list = query.fetch_all(conn).await?;

    Ok(status_list)
}

pub async fn get_task_status(conn: &sqlx::PgPool, pk: &i64) -> Result<TaskStatus, Box<dyn Error>> {
    let q = "SELECT * FROM task_status WHERE id = $1";
    let query = sqlx::query_as::<_, TaskStatus>(q).bind(&pk);

    let result = query.fetch_one(conn).await?;

    Ok(result)
}

pub async fn create_task_status(conn: &sqlx::PgPool, name: String) -> Result<TaskStatus, Box<dyn Error>> {
    let q = r"
        INSERT INTO task_status (
            name
        ) VALUES ($1)
        RETURNING *
    ";
    let query = sqlx::query_as::<_, TaskStatus>(q).bind(name);

    let result = query.fetch_one(conn).await?;

    Ok(result)
}
