use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::employee_model::{CreateEmployee, Employee};

pub async fn create_employee(pool: &SqlitePool, employee: CreateEmployee) -> Result<Employee> {
    let response =
        sqlx::query_as::<_, Employee>("INSERT INTO employee (name) VALUES ($1) RETURNING id, name")
            .bind(employee.name)
            .fetch_one(pool)
            .await?;
    Ok(response)
}

pub async fn all_employees(pool: &SqlitePool) -> Result<Vec<Employee>> {
    let response = sqlx::query_as::<_, Employee>("SELECT * FROM employee")
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn employee_by_id(pool: &SqlitePool, id: i32) -> Result<Employee> {
    let response = sqlx::query_as::<_, Employee>("SELECT * FROM employee WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(response)
}

pub async fn update_employee(pool: &SqlitePool, employee: &Employee) -> Result<Employee> {
    let response = sqlx::query_as::<_, Employee>(
        "UPDATE employee SET name = $1 WHERE id = $2 RETURNING id, name",
    )
    .bind(&employee.name)
    .bind(employee.id)
    .fetch_one(pool)
    .await?;
    Ok(response)
}

// FIX: This is not working.
pub async fn delete_employee(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM employee WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
