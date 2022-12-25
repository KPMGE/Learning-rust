use std::io::{Error, ErrorKind};
use std::str::FromStr;

use warp::Filter;

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

#[tokio::main]
async fn main() {
  let hi = warp::path("hello").map(|| format!("Hello world"));
  println!("listening on: http://locahost:3333");
  warp::serve(hi).run(([127, 0, 0, 1], 3333)).await;
}
