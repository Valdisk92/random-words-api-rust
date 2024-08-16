use serde::{Deserialize, Serialize};
use surrealdb::{engine::{local::Db, remote::ws::Client}, sql::Thing, Surreal};

use super::model::WordList as WordListModel;

pub async fn find_all(db: &Surreal<Db>) -> surrealdb::Result<Vec<WordListModel>> {
    let list_words: Vec<WordListRecord> = db.select("word_lists").await?;
    let list_words = list_words
        .into_iter()
        .map(|item| WordListModel {
            id: Some(item.id.id.to_string()),
            name: item.name,
            words: item.words,
        })
        .collect();

    Ok(list_words)
}

pub async fn create(db: &Surreal<Db>, word_list: WordListModel) -> surrealdb::Result<()> {
    let created: Vec<WordListRecord> = db
        .create("word_lists")
        .content(WordList {
            name: word_list.name,
            words: word_list.words,
        })
        .await?;

    dbg!(created);

    Ok(())
}

pub async fn update(db: &Surreal<Db>, word_list: WordListModel) -> surrealdb::Result<()> {
    let updated: Option<WordListRecord> = db
        .update(("word_lists", word_list.id.unwrap()))
        .content(WordList {
            name: word_list.name,
            words: word_list.words,
        })
        .await?;

    dbg!(updated);

    Ok(())
}

pub async fn find_one(
    db: &Surreal<Db>,
    id: String,
) -> surrealdb::Result<Option<WordListModel>> {
    let word_list: Option<WordListRecord> = db.select(("word_lists", id)).await?;

    let word_list = word_list.map(|record| WordListModel {
        id: Some(record.id.id.to_string()),
        name: record.name,
        words: record.words,
    });

    Ok(word_list)
}

pub async fn delete(db: &Surreal<Db>, id: String) -> surrealdb::Result<()> {
    let _: Option<WordListRecord> = db.delete(("word_lists", id)).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct WordListRecord {
    id: Thing,
    name: String,
    words: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WordList {
    name: String,
    words: Vec<String>,
}
