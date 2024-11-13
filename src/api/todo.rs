use actix_web::{get, post, web, HttpResponse, Responder, patch, delete};
use chrono::{DateTime, Local, Utc};
use serde_json::json;

use crate::models::todo::{TodoModel, TodoModelResponse};
use crate::schemas::todo::{CreateTodoSchema, FilterOptions, UpdateTodoSchema};
use crate::AppState;

/// A simple health check endpoint that responds with a success message.
///
/// The endpoint is used to verify that the server is running and all services
/// are operational. It returns a JSON object with a status and a message.
///
/// GET /healthchecker
///
/// # Example
///
/// ```sh
/// $ curl -X GET http://localhost:8080/healthchecker
/// ```
///
/// # Response
///
/// ```json
/// {
///     "status": "success",
///     "message": "Rust, SQLX, MySQL, Actix Web todo is Good🏃‍♂️"
/// }
/// ```
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    // Define a constant message to be returned in the response
    const MESSAGE: &str = "Rust, SQLX, MySQL, Actix Web todo is Good🏃‍♂️";

    // Return an HTTP response with status 200 (OK) and a JSON body
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

/// Retrieves a paginated list of todo items.
/// 
/// GET /todos
/// 
/// # Query Parameters
/// - `page`: Optional page number for pagination (default is 1).
/// - `limit`: Optional number of items per page (default is 10).
/// 
/// # Response
/// Returns a JSON response containing the status, number of results, and a list of todos.
/// 
/// # Example
/// 
/// $ curl -X GET "http://localhost:8080/todos?page=1&limit=10"
/// 
/// ```json
/// {
///     "status": "success",
///     "results": 10,
///     "todos": [
///         // List of todo items
///     ]
/// }
/// ```
#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<FilterOptions>, // Query parameters for pagination
    data: web::Data<AppState>, // Application state containing database connection pool
) -> impl Responder {
    // Set default pagination values if not provided
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Fetch todos from the database with pagination
    let todos: Vec<TodoModel> = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos ORDER by id desc LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    // Transform database records into response-friendly format
    let todo_responses = todos
        .into_iter()
        .map(|todo| filter_db_record(&todo))
        .collect::<Vec<TodoModelResponse>>();

    // Create JSON response
    let json_response = json!({
        "status": "success",
        "results": todo_responses.len(),
        "todos": todo_responses
    });

    // Return HTTP response
    HttpResponse::Ok().json(json_response)
}

#[post("/todos")]
async fn create_todo_handler(
    body: web::Json<CreateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query(
        r#"INSERT INTO todos (title,contents,created_at,updated_at,is_completed,is_deleted) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
        .bind(body.title.to_owned().unwrap_or_default())
        .bind(body.contents.to_owned().unwrap_or_default())
        .bind(body.created_at.unwrap_or(DateTime::from(Local::now())))
        .bind(body.updated_at.unwrap_or(DateTime::from(Local::now())))
        .bind(body.is_completed.to_owned().unwrap_or("N".parse().unwrap()))
        .bind(body.is_deleted.to_owned().unwrap_or(String::from("N")))
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

#[get("/todos/{id}")]
async fn get_todo_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner().to_string();
    let query_result = sqlx::query_as!(TodoModel, r#"SELECT * FROM todos WHERE id = ?"#, todo_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(todo) => {
            let todo_response = json!({
                "status": "success",
                "data": json!({
                    "todo": filter_db_record(&todo)
                })
            });

            HttpResponse::Ok().json(todo_response)
        }
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(
                json!({"status": "fail","message": format!("todo with ID: {} not found", todo_id)}),
            )
        }
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

#[patch("/todos/{id}")]
async fn edit_todo_handler(
    path: web::Path<String>,
    body: web::Json<UpdateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner().to_string();
    let query_result = sqlx::query_as!(TodoModel, r#"SELECT * FROM todos WHERE id = ?"#, todo_id)
        .fetch_one(&data.db)
        .await;

    let todo = match query_result {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(
                json!({"status": "fail","message": format!("todo with ID: {} not found", todo_id)}),
            );
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let update_result = sqlx::query(
        r#"UPDATE todos SET title = ?, contents = ?, updated_at = ?, is_completed = ?, is_deleted = ? WHERE id = ?"#,
    )
        .bind(body.title.to_owned().unwrap_or_else(|| todo.title.unwrap().clone()))
        .bind(
            body.contents
                .to_owned()
                .unwrap_or_else(|| todo.contents.unwrap().clone()),
        )
        .bind::<&DateTime<Utc>>(&DateTime::from(Local::now()))
        .bind(
            body.is_completed
                .to_owned()
                .unwrap_or_else(|| todo.is_completed.clone().unwrap()),
        )
        .bind(
            body.is_deleted
                .to_owned()
                .unwrap_or_else(|| todo.is_deleted.clone().unwrap()),
        )
        .bind(todo_id.to_owned())
        .execute(&data.db)
        .await;

    match update_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("todo with ID: {} not found", todo_id);
                return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": message}));
        }
    }

    let updated_todo_result = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE id = ?"#,
        todo_id.to_owned()
    )
        .fetch_one(&data.db)
        .await;

    match updated_todo_result {
        Ok(todo) => {
            let todo_response = json!({"status": "success","data": json!({
                "todo": filter_db_record(&todo)
            })});

            HttpResponse::Ok().json(todo_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[patch("/toggle_complete/{id}")]
async fn toggle_complete(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner().to_string();
    let query_result = sqlx::query_as!(TodoModel, r#"SELECT * FROM todos WHERE id = ?"#, todo_id)
        .fetch_one(&data.db)
        .await;

    let todo = match query_result {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(
                json!({"status": "fail","message": format!("todo with ID: {} not found", todo_id)}),
            );
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let is_completed: Option<String> = todo.is_completed.clone();
    let update_flag: String = match is_completed {
        Some(str) => if str == "Y" {
            String::from("N")
        } else {
            String::from("Y")
        },
        None => String::from("Y")
    };
    println!("{:?}", &update_flag);

    let completed_at: Option<DateTime<Utc>> = if update_flag == "Y" {
        Some(DateTime::from(Local::now()))
    } else {
        None
    };

    let update_result = sqlx::query(
        r#"UPDATE todos SET updated_at = ?, is_completed = ?, completed_at = ? WHERE id = ?"#,
    )
        .bind::<DateTime<Utc>>(DateTime::from(Local::now()))
        .bind(
            &update_flag
        )
        .bind(
            &completed_at
        )
        .bind(todo_id.to_owned())
        .execute(&data.db)
        .await;

    match update_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("todo with ID: {} not found", todo_id);
                return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": message}));
        }
    }

    let updated_todo_result = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE id = ?"#,
        todo_id.to_owned()
    )
        .fetch_one(&data.db)
        .await;

    match updated_todo_result {
        Ok(todo) => {
            let todo_response = json!({"status": "success","data": json!({
                "todo": filter_db_record(&todo)
            })});

            HttpResponse::Ok().json(todo_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[delete("/todos/{id}")]
async fn delete_todo_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner().to_string();
    let query_result = sqlx::query!(r#"DELETE FROM todos WHERE id = ?"#, todo_id)
        .execute(&data.db)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Todo with ID: {} not found", todo_id);
                HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error","message": message}))
        }
    }
}


fn filter_db_record(todo: &TodoModel) -> TodoModelResponse {
    TodoModelResponse {
        id: todo.id,
        title: todo.title.clone().unwrap_or_default(),
        contents: todo.contents.clone().unwrap_or_default(),
        created_at: todo.created_at,  // UTC 시간 그대로 유지
        updated_at: todo.updated_at,  // UTC 시간 그대로 유지
        completed_at: todo.completed_at,  // UTC 시간 그대로 유지
        is_completed: todo.is_completed.clone().unwrap_or_default(),
        is_deleted: todo.is_deleted.clone().unwrap_or_default(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/todo")
        .service(health_checker_handler)
        .service(todo_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler)
        .service(toggle_complete);

    conf.service(scope);
}
