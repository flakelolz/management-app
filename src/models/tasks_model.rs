use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq, Debug)]
pub struct Tasks {
    pub id: i32,
    pub project_id: i32,
    pub employee_id: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct CreateTask {
    pub project_id: i32,
    pub employee_id: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UpdateTask {
    pub project_id: i32,
    pub employee_id: i32,
}
