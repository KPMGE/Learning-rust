use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
  pub id: QuestionId,
  pub title: String,
  pub content: String,
  pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct QuestionId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewQuestion {
  pub title: String,
  pub content: String,
  pub tags: Option<Vec<String>>,
}

