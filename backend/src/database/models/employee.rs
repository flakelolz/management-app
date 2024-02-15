use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Employee {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct CreateEmployee {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UpdateEmployee {
    pub name: String,
}
