use axum::{
    routing::{delete, get, post},
    Router,
};
use msg::routes::{create_message, delete_message, root};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

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
        .route("/messages", post(create_message))
        .route("/messages/:message_id", delete(delete_message))
        .nest_service("/dist", ServeDir::new("dist"))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
