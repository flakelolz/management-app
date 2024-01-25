use axum::extract::{self, Path};
use axum::routing::{delete, post, put};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::project_controller;
use crate::models::project_model::{CreateProject, Project};

pub fn projects_api() -> Router {
    Router::new()
        .route("/", get(get_all_projects))
        .route("/", post(add_project))
        .route("/", put(update_project))
        .route("/:id", get(project_by_id))
        .route("/:id", delete(delete_project))
}

async fn get_all_projects(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    if let Ok(projects) = project_controller::all_projects(&cnn).await {
        Ok(Json(projects))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn project_by_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Project>, StatusCode> {
    if let Ok(project) = project_controller::project_by_id(&cnn, id).await {
        Ok(Json(project))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_project(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_project): extract::Json<CreateProject>,
) -> Result<Json<Project>, StatusCode> {
    if let Ok(project) = project_controller::create_project(&cnn, create_project).await {
        Ok(Json(project))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn update_project(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(project): extract::Json<Project>,
) -> Result<Json<Project>, StatusCode> {
    if let Ok(project) = project_controller::update_project(&cnn, &project).await {
        Ok(Json(project))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn delete_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    if let Ok(()) = project_controller::delete_project(&cnn, id).await {
        Ok(())
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
