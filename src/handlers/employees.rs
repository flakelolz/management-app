use axum::extract::{self, Path};
use axum::{http::StatusCode, routing::get, Router};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::database::models::employee::*;
use crate::database::{employees, tasks};

pub fn employees_api() -> Router {
    Router::new()
        .route("/", get(get_all_employees).post(add_employee))
        .route(
            "/:id",
            get(get_employee)
                .put(update_employee)
                .delete(delete_employee),
        )
}

async fn get_all_employees(
    Extension(cnn): Extension<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<Employee>>), StatusCode> {
    if let Ok(employees) = employees::get_all_employees(&cnn).await {
        Ok((StatusCode::OK, Json(employees)))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn get_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
) -> Result<(StatusCode, Json<Employee>), (StatusCode, Json<String>)> {
    match employees::get_employee_by_id(&cnn, employee_id).await {
        Ok(employee) => Ok((StatusCode::OK, Json(employee))),
        Err(e) => {
            println!("get_employee ERROR: {:?}", e);
            Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    }
}

async fn add_employee(
    Extension(cnn): Extension<SqlitePool>,
    extract::Json(create_employee): extract::Json<CreateEmployee>,
) -> Result<(StatusCode, Json<Employee>), (StatusCode, Json<String>)> {
    match employees::create_employee(&cnn, create_employee).await {
        Ok(employee) => Ok((StatusCode::CREATED, Json(employee))),
        Err(e) => {
            println!("add_employee ERROR: {:?}", e);
            Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    }
}

async fn update_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
    extract::Json(employee): extract::Json<UpdateEmployee>,
) -> Result<(StatusCode, Json<Employee>), (StatusCode, Json<String>)> {
    match employees::update_employee(&cnn, employee_id, &employee).await {
        Ok(employee) => Ok((StatusCode::OK, Json(employee))),
        Err(e) => {
            println!("update_employee ERROR: {:?}", e);
            Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    }
}

async fn delete_employee(
    Extension(cnn): Extension<SqlitePool>,
    Path(employee_id): Path<i32>,
) -> Result<(StatusCode, Json<Employee>), (StatusCode, Json<String>)> {
    match tasks::delete_all_employee_tasks(&cnn, employee_id).await {
        Ok(_) => (),
        Err(e) => println!("delete_all_employee_tasks ERROR: {:?}", e),
    }

    match employees::delete_employee(&cnn, employee_id).await {
        Ok(employee) => Ok((StatusCode::OK, Json(employee))),
        Err(e) => {
            println!("delete_employee ERROR: {:?}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(format!("Employee with the id {employee_id} does not exist")),
            ))
        }
    }
}
