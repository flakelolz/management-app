use axum::extract::{self, Path};
use axum::routing::{delete, post, put};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::controllers::employee_controller;
use crate::models::employee_model::*;

pub fn employees_api() -> Router {
    Router::new()
        .route("/", get(get_all_employees))
        .route("/", post(add_employee))
        .route("/", put(update_employee))
        .route("/:id", get(get_employee))
        .route("/:id", delete(delete_employee))
}
async fn get_all_employees(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    if let Ok(employees) = employee_controller::all_employees(&cnn).await {
        Ok(Json(employees))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Employee>, StatusCode> {
    if let Ok(employee) = employee_controller::employee_by_id(&cnn, id).await {
        Ok(Json(employee))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_employee(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_employee): extract::Json<CreateEmployee>,
) -> Result<Json<Employee>, StatusCode> {
    if let Ok(employee) = employee_controller::create_employee(&cnn, create_employee).await {
        Ok(Json(employee))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn update_employee(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(employee): extract::Json<Employee>,
) -> Result<Json<Employee>, StatusCode> {
    if let Ok(employee) = employee_controller::update_employee(&cnn, &employee).await {
        Ok(Json(employee))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn delete_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match employee_controller::delete_employee(&cnn, id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("Error: {:?}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

