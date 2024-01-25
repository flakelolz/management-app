use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    // pub assigned_to: i32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct CreateProject {
    pub name: String,
}
