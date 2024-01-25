use axum::{
    extract::{self, Path},
    http::StatusCode,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use sqlx::SqlitePool;

use crate::models::employee_model::*;
use crate::controllers::employee_controller;

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
    extract::Json(employee): extract::Json<CreateEmployee>,
) -> Result<Json<Employee>, StatusCode> {
    if let Ok(employee) = employee_controller::create_employee(&cnn, employee).await {
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

// FIX: This is not working.
async fn delete_employee(Extension(cnn): Extension<SqlitePool>, Path(id): Path<i32>) -> StatusCode {
    if employee_controller::delete_employee(&cnn, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

