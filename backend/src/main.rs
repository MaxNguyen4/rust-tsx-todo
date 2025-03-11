use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
};

use serde::{Deserialize, Serialize};

use std::sync::Arc;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use tower_http::cors::{Any, CorsLayer};

use chrono::NaiveDate;

pub struct AppState {
    pg_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    // Establish database connection
    let database_url = "postgres://maxnguyen:password@localhost:5432/maxnguyen";
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to database successful");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Create cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application with single route
    let app_state = Arc::new(AppState {
        pg_pool: pool.clone(),
    });
    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/post-todo", post(post_todo))
        .route("/delete-todo", delete(delete_todo))
        .layer(cors)
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: i32,
    user_id: i32,
    todo: String,
    category: Option<String>,
    deadline: Option<NaiveDate>,
}

async fn get_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    match sqlx::query_as!(
        Todo,
        "SELECT id, user_id, todo, category, deadline FROM todos"
    )
    .fetch_all(&state.pg_pool)
    .await
    {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
struct CreateTodoReq {
    user_id: i32,
    todo: String,
    category: Option<String>,
    deadline: Option<NaiveDate>,
}

async fn post_todo(
    State(state): State<Arc<AppState>>,
    Json(todo_req): Json<CreateTodoReq>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    match sqlx::query_as!(
        Todo,
        "INSERT INTO todos (user_id, todo, category, deadline)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, todo, category, deadline",
        todo_req.user_id,
        todo_req.todo,
        todo_req.category,
        todo_req.deadline
    )
    .fetch_one(&state.pg_pool)
    .await
    {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_todo() {}
