use axum::extract::{Form, Path, State};
use axum_macros::debug_handler;
use sqlx::SqlitePool;

use crate::{
    markup::{IndexTemplate, MessageItemTemplate},
    CreateMessage,
};
use crate::{AppResult, Message};

pub async fn root(State(pool): State<SqlitePool>) -> AppResult<IndexTemplate> {
    let messages = sqlx::query_as!(Message, "select * from messages")
        .fetch_all(&pool)
        .await?;

    Ok(IndexTemplate { messages })
}

#[debug_handler]
pub async fn create_message(
    State(pool): State<SqlitePool>,
    Form(input): Form<CreateMessage>,
) -> AppResult<MessageItemTemplate> {
    let content = input.content;
    // Insert the task, then obtain the ID of this row
    let new_message = sqlx::query_as!(
        Message,
        r#"
INSERT INTO messages ( content )
VALUES ( ?1 ) RETURNING *
        "#,
        content
    )
    .fetch_one(&pool)
    .await?;

    Ok(MessageItemTemplate {
        message: new_message,
    })
}

#[debug_handler]
pub async fn delete_message(
    State(pool): State<SqlitePool>,
    Path(message_id): Path<String>,
) -> AppResult<()> {
    sqlx::query!(
        r#"
DELETE FROM messages WHERE id = ?1
        "#,
        message_id
    )
    .execute(&pool)
    .await?;

    Ok(())
}
