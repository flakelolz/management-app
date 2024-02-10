use axum::extract::{self, Path};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::{project_controller, tasks_controller};
use crate::models::project_model::{CreateProject, Project, UpdateProject};

pub fn projects_api() -> Router {
    Router::new()
        .route("/", get(get_all_projects).post(add_project))
        .route(
            "/:id",
            get(get_project).put(update_project).delete(delete_project),
        )
}

async fn get_all_projects(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<Project>>), (StatusCode, Json<String>)> {
    match project_controller::all_projects(&cnn).await {
        Ok(projects) => Ok((StatusCode::OK, Json(projects))),
        Err(e) => {
            println!("get_all_projects ERROR: {:?}", e);
            Err((StatusCode::SERVICE_UNAVAILABLE, Json(e.to_string())))
        }
    }
}

async fn get_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<String>)> {
    match project_controller::project_by_id(&cnn, project_id).await {
        Ok(project) => Ok((StatusCode::OK, Json(project))),
        Err(e) => {
            println!("get_project ERROR: {:?}", e);
            Err((StatusCode::SERVICE_UNAVAILABLE, Json(e.to_string())))
        }
    }
}

async fn add_project(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_project): extract::Json<CreateProject>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<String>)> {
    match project_controller::create_project(&cnn, create_project).await {
        Ok(project) => Ok((StatusCode::CREATED, Json(project))),
        Err(e) => {
            println!("add_project ERROR: {:?}", e);
            Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    }
}

async fn update_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
    extract::Json(project): extract::Json<UpdateProject>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<String>)> {
    match project_controller::update_project(&cnn, project_id, &project).await {
        Ok(project) => Ok((StatusCode::OK, Json(project))),
        Err(e) => {
            println!("update_project ERROR: {:?}", e);
            Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    }
}

async fn delete_project(
    Extension(cnn): Extension<SqlitePool>,
    Path(project_id): Path<i32>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<String>)> {
    match tasks_controller::delete_all_project_tasks(&cnn, project_id).await {
        Ok(_) => (),
        Err(e) => println!("delete_all_project_tasks ERROR: {:?}", e),
    }

    match project_controller::delete_project(&cnn, project_id).await {
        Ok(project) => Ok((StatusCode::OK, Json(project))),
        Err(e) => {
            println!("delete_project ERROR: {:?}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(format!("Project with the id {project_id} does not exist")),
            ))
        }
    }
}
