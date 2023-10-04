use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // set up connection pool
    let pool = SqlitePool::connect("msg.db")
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/messages", get(fetch_messages))
        .route("/messages", post(create_message))
        .route("/messages/:message_id", delete(delete_message))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "messages.html")]
struct MessagesTemplate {
    messages: Vec<Message>,
}

async fn root() -> HelloTemplate<'static> {
    HelloTemplate { name: "Justin" }
}

async fn fetch_messages(State(pool): State<SqlitePool>) -> MessagesTemplate {
    let records = sqlx::query_as!(Message, "select * from messages")
        .fetch_all(&pool)
        .await
        .unwrap();

    // Json(records)
    MessagesTemplate { messages: records }
}

#[derive(Debug, Deserialize)]
struct CreateMessage {
    content: String,
}

#[derive(Debug, Serialize)]
struct Message {
    id: i64,
    content: String,
}

#[debug_handler]
async fn create_message(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateMessage>,
) -> Result<String, AppError> {
    let content = input.content;
    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO messages ( content )
VALUES ( ?1 )
        "#,
        content
    )
    .execute(&pool)
    .await?
    .last_insert_rowid();

    Ok(id.to_string())
}

#[debug_handler]
async fn delete_message(
    State(pool): State<SqlitePool>,
    Path(message_id): Path<String>,
) -> Result<String, AppError> {
    let id = sqlx::query!(
        r#"
DELETE FROM messages WHERE id = ?1
        "#,
        message_id
    )
    .execute(&pool)
    .await?
    .rows_affected();

    Ok(id.to_string())
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
