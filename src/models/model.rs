use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoModel {
    pub id: u64,
    pub title: Option<String>,
    pub contents: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_completed: Option<String>,
    pub is_deleted: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TodoModelResponse {
    pub id: u64,
    pub title: String,
    pub contents: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_completed: String,
    pub is_deleted: String
}