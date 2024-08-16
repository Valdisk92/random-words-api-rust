use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use surrealdb::{
    engine::local::{Db, RocksDb},
    opt::{auth::Root, Config},
    Surreal,
};
use word_lists::api_handlers::{
    add_word_to_word_list_handler, create_word_list_handler, delete_word_list_handler,
    find_all_handler, remove_word_from_word_list_handler,
};

mod errors;
mod word_lists;

#[derive(Clone)]
pub struct AppState {
    db: Arc<Surreal<Db>>,
}

#[tokio::main]
async fn main() {
    let db = Surreal::new::<RocksDb>("./database").await.unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.unwrap();

    //Create global state with db connection
    let state = AppState { db: Arc::new(db) };

    //Create router
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/api/v1/word-lists", get(find_all_handler))
        .route("/api/v1/word-lists", post(create_word_list_handler))
        .route(
            "/api/v1/word-lists/:word_list_id",
            delete(delete_word_list_handler),
        )
        .route(
            "/api/v1/word-lists/:word_list_id/words",
            post(add_word_to_word_list_handler),
        )
        .route(
            "/api/v1/word-lists/:word_list_id/words/:word_to_remove",
            delete(remove_word_from_word_list_handler),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
