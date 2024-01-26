use axum::extract::{self, Path};
use axum::routing::{delete, post, put};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::{project_controller, tasks_controller};
use crate::models::project_model::{CreateProject, Project, UpdateProject};

pub fn projects_api() -> Router {
    Router::new()
        .route("/", get(get_all_projects))
        .route("/", post(add_project))
        .route("/:id", get(get_project))
        .route("/:id", put(update_project))
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

async fn get_project(
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
    match project_controller::create_project(&cnn, create_project).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => {
            println!("Error: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn update_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
    extract::Json(project): extract::Json<UpdateProject>,
) -> Result<Json<Project>, StatusCode> {
    if let Ok(project) = project_controller::update_project(&cnn, id, &project).await {
        Ok(Json(project))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn delete_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let tasks = tasks_controller::task_by_project_id(&cnn, id).await;
    match tasks {
        Ok(tasks) => {
            for task in tasks {
                match tasks_controller::delete_task(&cnn, task.id).await {
                    Ok(_) => (),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    match project_controller::delete_project(&cnn, id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("Error: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
