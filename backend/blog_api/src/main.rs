use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
struct Question {
  id: QuestionId,
  title: String,
  content: String,
  tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl FromStr for QuestionId {
  type Err = Error;

  fn from_str(id: &str) -> Result<Self, Self::Err> {
    match id.is_empty() {
      false => Ok(QuestionId(id.to_string())),
      true => Err(Error::new(ErrorKind::InvalidInput, "No id provided"))
    }
  }
}

impl Question {
  fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
    Question {
      id,
      title,
      content,
      tags,
    }
  }
}

fn main() {
  let q1 = Question::new(
    QuestionId::from_str("1234").expect("No id provided"),
    "What's the answer for the life, the universe and everything?".to_string(),
    "some content".to_string(),
    Some(vec!["faq".to_string(), "fiction".to_string()]),
  );

  println!("{:?}", q1);
}
