use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::Serialize;

use warp::hyper::StatusCode;
use warp::{Filter, Rejection, Reply};
use warp::reject::Reject;

#[derive(Debug, Serialize)]
struct Question {
  id: QuestionId,
  title: String,
  content: String,
  tags: Option<Vec<String>>,
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

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn invalid_id_error_handler(r: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(InvalidId) = r.find() {
    Ok(warp::reply::with_status("No valid id presented", StatusCode::UNPROCESSABLE_ENTITY))
  } else {
    Ok(warp::reply::with_status("Route not found", StatusCode::NOT_FOUND))
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

  match question.id.0.is_empty() {
    true => Err(warp::reject::custom(InvalidId)),
    false => Ok(warp::reply::json(&question))
  }
}

#[tokio::main]
async fn main() {
  let get_questions_route = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and_then(get_questions)
    .recover(invalid_id_error_handler);

  let routes = get_questions_route;

  println!("Listening on: http://locahost:3333");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
