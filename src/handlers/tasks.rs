use axum::extract::{self, Path};
use axum::routing::delete;
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::tasks_controller;
use crate::models::tasks_model::*;

pub fn assigned_api() -> Router {
    Router::new()
        .route("/", get(get_all_tasks).post(create_task))
        .route(
            "/:id",
            get(get_task_by_id).put(update_task).delete(delete_task),
        )
        .route("/employee/:id", get(get_task_by_employee_id))
        .route("/employee/:id/not", get(get_not_assigned_tasks))
        .route("/project/:id", get(get_task_by_project_id))
        .route(
            "/employee/:id/project/:id",
            delete(delete_task_by_employee_passing_id),
        )
}

async fn get_all_tasks(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<Json<Vec<Tasks>>, StatusCode> {
    match tasks_controller::all_tasks(&cnn).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(e) => {
            println!("get_all_tasks ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_task_by_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Tasks>, StatusCode> {
    match tasks_controller::task_by_id(&cnn, id).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => {
            println!("get_task_by_id ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_task_by_employee_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Tasks>>, StatusCode> {
    match tasks_controller::task_by_employee_id(&cnn, id).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => {
            println!("get_task_by_employee_id ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_task_by_project_id(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Tasks>>, StatusCode> {
    match tasks_controller::task_by_project_id(&cnn, id).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => {
            println!("get_task_by_project_id ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

// FIX: Not working how I intend it to
async fn get_not_assigned_tasks(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
) -> Result<Json<Vec<Tasks>>, StatusCode> {
    match tasks_controller::task_not_assigned(&cnn, employee_id).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => {
            println!("get_task_by_project_id ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn create_task(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_task): extract::Json<CreateTask>,
) -> Result<(StatusCode, Json<Tasks>), StatusCode> {
    match tasks_controller::create_task(&cnn, create_task).await {
        Ok(task) => Ok((StatusCode::CREATED, Json(task))),
        Err(e) => {
            println!("create_task ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn update_task(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
    extract::Json(update_task): extract::Json<UpdateTask>,
) -> Result<Json<Tasks>, StatusCode> {
    match tasks_controller::update_task(&cnn, id, &update_task).await {
        Ok(task) => Ok(Json(task)),
        Err(e) => {
            println!("update_task ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn delete_task(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Tasks>), StatusCode> {
    match tasks_controller::delete_task(&cnn, id).await {
        Ok(task) => Ok((StatusCode::OK, Json(task))),
        Err(e) => {
            println!("delete_task ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn delete_task_by_employee_passing_id(
    Extension(cnn): Extension<SqlitePool>,
    Path((employee_id, project_id)): Path<(i32, i32)>,
) -> Result<(StatusCode, Json<Vec<Tasks>>), StatusCode> {
    match tasks_controller::delete_task_by_employee_and_project_id(&cnn, employee_id, project_id)
        .await
    {
        Ok(tasks) => Ok((StatusCode::OK, Json(tasks))),
        Err(e) => {
            println!("delete_task_by_employee_passing_id ERROR: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
