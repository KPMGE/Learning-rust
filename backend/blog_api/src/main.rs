use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::{Serialize, Deserialize};

use warp::cors::CorsForbidden;
use warp::hyper::{StatusCode, Method};
use warp::{Filter, Rejection, Reply};

use std::collections::HashMap;

#[derive(Clone)]
struct Store {
  questions: HashMap<QuestionId, Question>
}

impl Store {
  fn new() -> Self {
    Store { 
      questions: Self::init()
    }
  }

  fn init() -> HashMap<QuestionId, Question> {
    let file = include_str!("../questions.json");
    serde_json::from_str(file).expect("could not read file questions.json")
  }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
  id: QuestionId,
  title: String,
  content: String,
  tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
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

async fn error_handler(r: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(error) = r.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(error.to_string(), StatusCode::FORBIDDEN))
  } else {
    Ok(warp::reply::with_status("Route not found".to_string(), StatusCode::NOT_FOUND))
  }
}

async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
  println!("params: {:?}", params);

  let mut start = 0;

  if let Some(n) = params.get("start") {
    start = n.parse::<usize>().expect("could not parse params start as a number");
  }

  println!("start: {}", start);

  let res: Vec<Question> = store.questions.values().cloned().collect();
  Ok(warp::reply::json(&res))
}

#[tokio::main]
async fn main() {
  let store = Store::new();
  let store_filter = warp::any().map(move || store.clone());

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::DELETE]);

  let get_questions_route = warp::get()
    .and(warp::path("questions"))
    .and(warp::query())
    .and(warp::path::end())
    .and(store_filter)
    .and_then(get_questions)
    .recover(error_handler);

  let routes = get_questions_route.with(cors);

  println!("Listening on: http://locahost:3333");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
