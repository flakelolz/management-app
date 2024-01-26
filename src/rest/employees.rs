use axum::extract::{self, Path};
use axum::routing::{delete, post, put};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::{employee_controller, tasks_controller};
use crate::models::employee_model::*;

pub fn employees_api() -> Router {
    Router::new()
        .route("/", get(get_all_employees))
        .route("/", post(add_employee))
        .route("/:id", get(get_employee))
        .route("/:id", put(update_employee))
        .route("/:id", delete(delete_employee))
}
async fn get_all_employees(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    if let Ok(employees) = employee_controller::get_all_employees(&cnn).await {
        Ok(Json(employees))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
) -> Result<Json<Employee>, StatusCode> {
    match employee_controller::get_employee_by_id(&cnn, employee_id).await {
        Ok(employee) => Ok(Json(employee)),
        Err(e) => {
            println!("get_employee ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn add_employee(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_employee): extract::Json<CreateEmployee>,
) -> Result<Json<Employee>, StatusCode> {
    match employee_controller::create_employee(&cnn, create_employee).await {
        Ok(employee) => Ok(Json(employee)),
        Err(e) => {
            println!("add_employee ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn update_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
    extract::Json(employee): extract::Json<UpdateEmployee>,
) -> Result<Json<Employee>, StatusCode> {
    match employee_controller::update_employee(&cnn, employee_id, &employee).await {
        Ok(employee) => Ok(Json(employee)),
        Err(e) => {
            println!("update_employee ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn delete_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let tasks = tasks_controller::task_by_employee_id(&cnn, employee_id).await;
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

    match employee_controller::delete_employee(&cnn, employee_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("delete_employee ERROR: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

