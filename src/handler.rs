use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
// use sqlx::__rt::yield_now;

use crate::models::model::{TodoModel, TodoModelResponse};
use crate::schema::CreateTodoSchema;
use crate::{schema::FilterOptions, AppState};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Rust, SQLX, MySQL, Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<TodoModel> = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos ORDER by id desc LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let todo_responses = todos
        .into_iter()
        .map(|todo| filter_db_record(&todo))
        .collect::<Vec<TodoModelResponse>>();

    let json_response = json!({
        "status": "success",
        "results": todo_responses.len(),
        "todos": todo_responses
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/todos/")]
async fn create_todo_handler(
    body: web::Json<CreateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // let user_id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(
        r#"INSERT INTO todos (title,contents,is_completed,is_deleted) VALUES (?, ?, ?, ?)"#,
    )
    .bind(body.title.to_owned().unwrap_or_default())
    .bind(body.contents.to_owned().unwrap_or_default())
    .bind(body.is_completed.to_owned().unwrap_or_default())
    // .bind(body.is_completed.to_owned().unwrap_or(yield_now().to_string()))
    .bind(body.is_deleted.to_owned().unwrap_or_default())
    // .bind(body.is_deleted.to_owned().unwrap_or(yield_now().to_string()))
    .execute(&data.db)
    .await;

    match query_result {
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),

        Ok(res) => {
            println!("Result OK");
            println!("{:?}", res);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "insert success",
                "data": format!("{:?}", res)
            }))
        }
    }
}
fn filter_db_record(todo: &TodoModel) -> TodoModelResponse {
    TodoModelResponse {
        id: todo.id.to_owned(),
        title: todo.title.to_owned().unwrap(),
        contents: todo.contents.to_owned().unwrap(),
        created_at: todo.created_at.unwrap(),
        updated_at: todo.updated_at.unwrap(),
        is_completed: todo.is_completed.to_owned().unwrap(),
        is_deleted: todo.is_deleted.to_owned().unwrap(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/todo")
        .service(health_checker_handler)
        .service(todo_list_handler)
        .service(create_todo_handler);

    conf.service(scope);
}
