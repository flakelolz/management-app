use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::project_model::{CreateProject, Project, UpdateProject};

pub async fn create_project(pool: &SqlitePool, project: CreateProject) -> Result<Project> {
    let response = sqlx::query_as::<_, Project>(
        "INSERT INTO project (name) VALUES ($1) RETURNING id, name",
    )
    .bind(project.name)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn all_projects(pool: &SqlitePool) -> Result<Vec<Project>> {
    let response = sqlx::query_as::<_, Project>("SELECT * FROM project")
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn project_by_id(pool: &SqlitePool, id: i32) -> Result<Project> {
    let response = sqlx::query_as::<_, Project>("SELECT * FROM project WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(response)
}

pub async fn update_project(pool: &SqlitePool, id: i32, project: &UpdateProject) -> Result<Project> {
    let response = sqlx::query_as::<_, Project>(
        "UPDATE project SET name = $1 WHERE id = $2 RETURNING id, name",
    )
    .bind(&project.name)
    // .bind(project.assigned_to)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn delete_project(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM project WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
