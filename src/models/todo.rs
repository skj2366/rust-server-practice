use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoModel {
    pub id: u64,
    pub title: Option<String>,
    pub contents: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: Option<String>,
    pub is_deleted: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TodoModelResponse {
    pub id: u64,
    pub title: String,
    pub contents: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: String,
    pub is_deleted: String
}