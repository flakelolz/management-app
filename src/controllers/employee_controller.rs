use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::employee_model::{CreateEmployee, Employee, UpdateEmployee};

pub async fn create_employee(pool: &SqlitePool, employee: CreateEmployee) -> Result<Employee> {
    let response =
        sqlx::query_as::<_, Employee>("INSERT INTO employee (name) VALUES ($1) RETURNING id, name")
            .bind(employee.name)
            .fetch_one(pool)
            .await?;
    Ok(response)
}

pub async fn get_all_employees(pool: &SqlitePool) -> Result<Vec<Employee>> {
    let response = sqlx::query_as::<_, Employee>("SELECT * FROM employee")
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn get_employee_by_id(pool: &SqlitePool, id: i32) -> Result<Employee> {
    let response = sqlx::query_as::<_, Employee>("SELECT * FROM employee WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(response)
}

pub async fn update_employee(pool: &SqlitePool, id: i32, employee: &UpdateEmployee) -> Result<Employee> {
    let response = sqlx::query_as::<_, Employee>(
        "UPDATE employee SET name = $1 WHERE id = $2 RETURNING id, name",
    )
    .bind(&employee.name)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

pub async fn delete_employee(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM employee WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
