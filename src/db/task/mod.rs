use sqlx::FromRow;
use std::error::Error;
use crate::db;
use crate::db::task_task_status::TaskTaskStatus;

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Task {
    pub fn to_api(&self) -> crate::api::task::Task {
        let name: String = String::from(&self.name);
        let description: Option<String> = match &self.description {
            Some(data) => Some(String::from(data)),
            None => None,
        };
        crate::api::task::Task {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name,
            description,
            start_date: self.start_date,
            end_date: self.end_date,
            status: vec![],
        }
    }
}

pub async fn get_task_list(conn: &sqlx::PgPool) -> Result<Vec<Task>, Box<dyn Error>> {
    let q = "SELECT * FROM tasks";
    let query = sqlx::query_as::<_, Task>(q);

    let task_list = query.fetch_all(conn).await?;

    Ok(task_list)
}

pub async fn get_task(conn: &sqlx::PgPool, pk: &i64) -> Result<Task, Box<dyn Error>> {
    let q = "SELECT * FROM tasks WHERE id = $1";
    let query = sqlx::query_as::<_, Task>(q).bind(pk);

    let task = query.fetch_one(conn).await?;

    Ok(task)
}

pub async fn create_task(
    conn: &sqlx::PgPool,
    name: String,
    description: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Task, Box<dyn Error>> {
    let q1 = r"
        INSERT INTO tasks (
            name,
            description,
            start_date,
            end_date
        ) VALUES ($1, $2, $3, $4)
        RETURNING *
    ";
    let query = sqlx::query_as::<_, Task>(q1)
        .bind(name)
        .bind(description)
        .bind(start_date)
        .bind(end_date);

    let new_task = query.fetch_one(conn).await?;

    let q2 = r"
        INSERT INTO tasks_task_status ('
            task_id,
            task_status_id
        ) VALUES
            ($1, $2),
            ($1, $3)
    ";
    let _ = sqlx::query_as::<_, TaskTaskStatus>(q2)
        .bind(new_task.id)
        .bind(db::task_status::StartingTaskStatusEntries::Created)
        .bind(db::task_status::StartingTaskStatusEntries::Open);

    Ok(new_task)
}

pub async fn update_task(
    conn: &sqlx::PgPool,
    id: i64,
    is_name: bool,
    name: Option<String>,
    is_description: bool,
    description: Option<String>,
    is_start_date: bool,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    is_end_date: bool,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Task, Box<dyn Error>> {
    let q = r"
        UPDATE tasks
        SET name = CASE WHEN ($1) THEN $2 ELSE name END,
            description = CASE WHEN ($3) THEN $4 ELSE description END,
            start_date = CASE WHEN ($5) THEN $6 ELSE start_date END,
            end_date = CASE WHEN ($7) THEN $8 ELSE end_date END
        WHERE id = $9
        RETURNING *
    ";
    let query = sqlx::query_as::<_, Task>(q)
        .bind(is_name)
        .bind(name)
        .bind(is_description)
        .bind(description)
        .bind(is_start_date)
        .bind(start_date)
        .bind(is_end_date)
        .bind(end_date)
        .bind(id);

    let new_task = query.fetch_one(conn).await?;

    Ok(new_task)
}
