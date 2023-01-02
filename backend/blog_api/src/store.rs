use sqlx::{PgPool, postgres::{PgPoolOptions, PgRow}, Row};

use crate::types::question::{Question, QuestionId, NewQuestion};

#[derive(Debug, Clone)]
pub struct Store {
  pub connection: PgPool,
}

impl Store {
  pub async fn new(db_url: &str) -> Self {
    let db_pool = match PgPoolOptions::new()
      .max_connections(5)
      .connect(db_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("could not establish DB connection!"),
      };

    Store { connection: db_pool }
  }

  pub async fn get_questions(self, limit: Option<i32>, offset: i32) -> Result<Vec<Question>, sqlx::Error> {
    match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2") 
      .bind(limit)
      .bind(offset)
      .map(|row: PgRow| Question{
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
      })
      .fetch_all(&self.connection).await {
        Ok(questions) => Ok(questions),
        Err(e) => Err(e),
      }
  }

  pub async fn add_question(self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
    match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags")
      .bind(new_question.title)
      .bind(new_question.content)
      .bind(new_question.tags)
      .map(|row: PgRow| Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
      })
      .fetch_one(&self.connection)
      .await {
        Ok(question) => Ok(question),
        Err(err) => Err(err),
      }
  }

  pub async fn update_question(self, question: Question, id: i32) -> Result<Question, sqlx::Error> {
    match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags")
      .bind(question.title)
      .bind(question.content)
      .bind(question.tags)
      .bind(id)
      .map(|row: PgRow| Question{
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
      })
      .fetch_one(&self.connection)
      .await {
        Ok(question) => Ok(question),
        Err(err) => Err(err),
      }
  }

  pub async fn delete_question(self, id: i32) -> Result<bool, sqlx::Error> {
    match sqlx::query("DELETE FROM questions WHERE id = $1")
      .bind(id)
      .execute(&self.connection)
      .await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
      }
  }
}
