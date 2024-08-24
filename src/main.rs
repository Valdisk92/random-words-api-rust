use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use word_lists::api_handlers::{
    add_word_to_word_list_handler, create_word_list_handler, delete_word_list_handler,
    find_all_handler, remove_word_from_word_list_handler,
};
use utoipa_swagger_ui::SwaggerUi;

mod errors;
mod word_lists;

#[derive(Clone)]
pub struct AppState {
    db: Arc<Surreal<Db>>,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "WordList", description = "WordList RESTful API for creating and retreiving word lists")
        ),
        security(
            ()
        ),
        paths(
            word_lists::api_handlers::find_all_handler,
            word_lists::api_handlers::create_word_list_handler,
            word_lists::api_handlers::add_word_to_word_list_handler,
            word_lists::api_handlers::remove_word_from_word_list_handler,
            word_lists::api_handlers::delete_word_list_handler,
        ),
        components(
            schemas(
                word_lists::model::WordList,
                word_lists::model::CreateWordListRequest,
                word_lists::model::AddWordToListRequest,
            )
        ),
        servers(
            (url = "http://localhost:3000", description = "Local server")
        )
    )]
    struct ApiDoc;

    println!("Starting web server!");
    let db = Surreal::new::<RocksDb>("./database").await.unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.unwrap();

    //Create global state with db connection
    let state = AppState { db: Arc::new(db) };

    //Create router
    let app = Router::new()
        .merge(SwaggerUi::new("/api-docs/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/api-docs/redoc", ApiDoc::openapi()))
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
    println!("Server started on port 3000");
    axum::serve(listener, app).await.unwrap();
}
