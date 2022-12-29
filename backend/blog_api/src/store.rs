use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::types::{
  answer::{Answer, AnswerId},
  question::{Question, QuestionId}
};

#[derive(Clone)]
pub struct Store {
  pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
  pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>
}

impl Store {
  pub fn new() -> Self {
    Store { 
      questions: Arc::new(RwLock::new(Self::init())),
      answers: Arc::new(RwLock::new(HashMap::new()))
    }
  }

  fn init() -> HashMap<QuestionId, Question> {
    let file = include_str!("../questions.json");
    serde_json::from_str(file).expect("could not read file questions.json")
  }
}