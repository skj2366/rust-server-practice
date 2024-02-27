use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    // #[serde(skip_serializing_if = "Option::DateTime<Utc>::now()")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    // #[serde(skip_serializing_if = "Option::DateTime<Utc>::now()")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_completed: Option<String>,
    pub is_deleted: Option<String>,
}