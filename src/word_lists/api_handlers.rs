use super::{
    model::{AddWordToListRequest, CreateWordListRequest, WordList},
    repository,
};
use crate::{errors::AppError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn find_all_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<WordList>>, AppError> {
    let word_lists = repository::find_all(&state.db).await?;
    Ok(Json(word_lists))
}

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

pub async fn delete_word_list_handler(
    Path(word_list_id): Path<String>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    repository::delete(&state.db, word_list_id).await?;

    Ok(())
}
