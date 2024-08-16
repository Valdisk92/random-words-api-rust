use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct WordList {
    pub id: Option<String>,
    pub name: String,
    pub words: Vec<String>,
}

impl WordList {
    pub fn add_word(&mut self, word: String) -> Result<(), AppError> {
        if self.words.contains(&word) {
            return Err(AppError::WordAlreadyExists);
        }
        self.words.push(word);

        Ok(())
    }

    pub fn remove_word(&mut self, word: String) -> Result<(), AppError> {
        match self.words.iter().position(|w| w == &word) {
            Some(position) => {
                self.words.swap_remove(position);
            }
            None => return Err(AppError::WordNotFound),
        };

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CreateWordListRequest {
    pub name: String,
    pub words: Vec<String>,
}

#[derive(Deserialize)]
pub struct AddWordToListRequest {
    pub word: String,
}