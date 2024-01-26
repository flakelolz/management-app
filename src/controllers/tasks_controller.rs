use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::tasks_model::{Tasks, CreateTask, UpdateTask};

pub async fn create_task(pool: &SqlitePool, assigned: CreateTask) -> Result<Tasks> {
    let response =
        sqlx::query_as::<_, Tasks>("INSERT INTO tasks (project_id, employee_id) VALUES ($1, $2) RETURNING id, project_id, employee_id")
            .bind(assigned.project_id)
            .bind(assigned.employee_id)
            .fetch_one(pool)
            .await?;
    Ok(response)
}

pub async fn all_tasks(pool: &SqlitePool) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks")
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn task_by_id(pool: &SqlitePool, id: i32) -> Result<Tasks> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(response)
}

pub async fn task_by_employee_id(pool: &SqlitePool, id: i32) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE employee_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn task_by_project_id(pool: &SqlitePool, id: i32) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn update_task(pool: &SqlitePool, id: i32, task: &UpdateTask) -> Result<Tasks> {
    let response = sqlx::query_as::<_, Tasks>(
        "UPDATE tasks SET project_id = $1, employee_id = $2 WHERE id = $3 RETURNING id, project_id, employee_id",
    )
    .bind(task.project_id)
    .bind(task.employee_id)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn delete_task(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

