use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::tasks_model::{CreateTask, Tasks, UpdateTask};

pub async fn create_task(pool: &SqlitePool, payload: CreateTask) -> Result<Tasks> {
    let response =
        sqlx::query_as::<_, Tasks>("INSERT INTO tasks (project_id, employee_id) VALUES ($1, $2) RETURNING id, project_id, employee_id")
            .bind(payload.project_id)
            .bind(payload.employee_id)
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

pub async fn task_by_id(pool: &SqlitePool, task_id: i32) -> Result<Tasks> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_one(pool)
        .await?;
    Ok(response)
}

pub async fn task_by_employee_id(pool: &SqlitePool, task_id: i32) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE employee_id = $1")
        .bind(task_id)
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn task_by_project_id(pool: &SqlitePool, task_id: i32) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(task_id)
        .fetch_all(pool)
        .await?;
    Ok(response)
}

// FIX: Not working how I intend it to
pub async fn task_not_assigned(pool: &SqlitePool, employee_id: i32) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>("SELECT * FROM tasks WHERE NOT employee_id = $1")
        .bind(employee_id)
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn update_task(pool: &SqlitePool, task_id: i32, task: &UpdateTask) -> Result<Tasks> {
    let response = sqlx::query_as::<_, Tasks>(
        "UPDATE tasks SET project_id = $1, employee_id = $2 WHERE id = $3 RETURNING id, project_id, employee_id",
    )
    .bind(task.project_id)
    .bind(task.employee_id)
    .bind(task_id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn delete_task(pool: &SqlitePool, task_id: i32) -> Result<Tasks> {
    let response = sqlx::query_as::<_, Tasks>(
        "DELETE FROM tasks WHERE id = $1 RETURNING id, project_id, employee_id",
    )
    .bind(task_id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn delete_all_employee_tasks(pool: &SqlitePool, employee_id: i32) -> Result<Vec<Tasks>> {
    let response =
        sqlx::query_as::<_, Tasks>("DELETE FROM tasks WHERE employee_id = $1 RETURNING *")
            .bind(employee_id)
            .fetch_all(pool)
            .await?;
    Ok(response)
}

pub async fn delete_task_by_employee_and_project_id(
    pool: &SqlitePool,
    employee_id: i32,
    project_id: i32,
) -> Result<Vec<Tasks>> {
    let response = sqlx::query_as::<_, Tasks>(
        "DELETE FROM tasks WHERE employee_id = $1 AND project_id = $2 RETURNING *",
    )
    .bind(employee_id)
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(response)
}
