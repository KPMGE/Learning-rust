use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::Serialize;

use warp::Filter;

#[derive(Debug, Serialize)]
struct Question {
  id: QuestionId,
  title: String,
  content: String,
  tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
  let question = Question::new(
    QuestionId::from_str("1234").expect("no id provided"),
    "the final question".to_string(),
    "what's the answer for the life, the universe and everything?".to_string(),
    Some(vec![
      "faq".to_string(),
      "universe".to_string()
    ])
  );

  Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
  let get_questions_route = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and_then(get_questions);

  let routes = get_questions_route;

  println!("Listening on: http://locahost:3333");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
