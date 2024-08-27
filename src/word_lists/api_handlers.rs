use super::{
    model::{AddWordToListRequest, CreateWordListRequest, WordList},
    repository,
};
use crate::{errors::AppError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};

/// List word lists
///
/// Returns list of all word lists stored in DB
#[utoipa::path(
    get,
    path = "/api/v1/word-lists",
    tag = "WordList", 
    responses(
        (status = 200, description = "Lis of all word lists", body = Vec<WordList>)
    ),
)]
pub async fn find_all_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<WordList>>, AppError> {
    let word_lists = repository::find_all(&state.db).await?;
    Ok(Json(word_lists))
}

/// Create word list
///
/// Creating new word list, generates unique identifier and stores in DB
#[utoipa::path(
    post,
    path = "/api/v1/word-lists",
    tag = "WordList",
    request_body = CreateWordListRequest,
    responses(
        (status = 200, description = "Empty 200 response if create was OK")
    ),
    )]
pub async fn create_word_list_handler(
    State(state): State<AppState>,
    Json(word_list_request): Json<CreateWordListRequest>,
) -> Result<(), AppError> {
    let word_list = WordList {
        id: Option::None,
        name: word_list_request.name,
        words: word_list_request.words,
    };

    repository::create(&state.db, word_list).await?;
    Ok(())
}

/// Add word to word list
///
/// Adding new word to word list. Returns errors if word already exists.
#[utoipa::path(
    post,
    path = "/api/v1/word-lists/{word_list_id}",
    tag = "WordList", 
    request_body = AddWordToListRequest,
    params(
        ("word_list_id" = String, Path, description = "Word list id")
    ),
    responses(
        (status = 200, description = "Empty 200 response if adding was OK"),
        (status = 404, body = String, description = "Returns 404 error if word list does not exist by provided id"),
        (status = 400, body = String, description = "Returns 400 BadRequest error if word already exist in word list")
    ),
    )]
pub async fn add_word_to_word_list_handler(
    Path(word_list_id): Path<String>,
    State(state): State<AppState>,
    Json(request_body): Json<AddWordToListRequest>,
) -> Result<(), AppError> {
    let word_list = repository::find_one(&state.db, word_list_id).await?;

    match word_list {
        Some(mut item) => {
            item.add_word(request_body.word)?;

            repository::update(&state.db, item).await?;
        }
        None => return Err(AppError::NotFound),
    }

    Ok(())
}

/// Remove word from word list
///
/// Removes word form word list by word_list_id and word. Returns error if word does not exist.
#[utoipa::path(
    delete,
    path = "/api/v1/word-lists/{word_list_id}/words/{word_to_remove}",
    tag = "WordList", 
    params(
        ("word_list_id" = String, Path, description = "Word list id"),
        ("word_to_remove" = String, Path, description = "Word to remove. Word must be URL encoded.")
    ),
    responses(
        (status = 200, description = "Empty 200 response if removing was OK"),
        (status = 404, body = String, description = "Returns 404 error if word list does not exist by provided id"),
        (status = 400, body = String, description = "Returns 400 BadRequest error if word does not exist in word list")
    ),
    )]
pub async fn remove_word_from_word_list_handler(
    Path((word_list_id, word_to_remove)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    let word_list = repository::find_one(&state.db, word_list_id).await?;

    match word_list {
        Some(mut item) => {
            item.remove_word(word_to_remove)?;

            repository::update(&state.db, item).await?;
        }
        None => return Err(AppError::NotFound),
    }

    Ok(())
}

/// Delete word list
///
/// Delete word list by id.
#[utoipa::path(
    delete,
    path = "/api/v1/word-lists/{word_list_id}",
    tag = "WordList", 
    params(
        ("word_list_id" = String, Path, description = "Word list id")
    ),
    responses(
        (status = 200, description = "Empty 200 response if deleting was OK"),
        (status = 404, body = String, description = "Returns 404 error if word list does not exist by provided id"),
    ),
    )]
pub async fn delete_word_list_handler(
    Path(word_list_id): Path<String>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    repository::delete(&state.db, word_list_id).await?;

    Ok(())
}
