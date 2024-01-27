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
    match project_controller::all_projects(&cnn).await {
        Ok(projects) => Ok(Json(projects)),
        Err(e) => {
            println!("get_all_projects ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn get_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
) -> Result<Json<Project>, StatusCode> {
    match project_controller::project_by_id(&cnn, project_id).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => {
            println!("get_project ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn add_project(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_project): extract::Json<CreateProject>,
) -> Result<Json<Project>, StatusCode> {
    match project_controller::create_project(&cnn, create_project).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => {
            println!("add_project ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn update_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
    extract::Json(project): extract::Json<UpdateProject>,
) -> Result<Json<Project>, StatusCode> {
    match project_controller::update_project(&cnn, project_id, &project).await {
        Ok(project) => Ok(Json(project)),
        Err(e) => {
            println!("update_project ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn delete_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let tasks = tasks_controller::task_by_project_id(&cnn, project_id).await;
    match tasks {
        Ok(tasks) => {
            for task in tasks {
                match tasks_controller::delete_task(&cnn, task.id).await {
                    Ok(_) => (),
                    Err(e) => println!("delete_task ERROR: {:?}", e),
                }
            }
        }
        Err(e) => println!("task_by_project_id ERROR: {:?}", e),
    }

    match project_controller::delete_project(&cnn, project_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("delete_project ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
