use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::hyper::{StatusCode, Method};
use warp::reject::Reject;
use warp::{Filter, Rejection, Reply};

use std::collections::HashMap;

#[derive(Debug)]
struct Pagination {
  start: usize,
  end: usize
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, ApiError> {
  if params.contains_key("start") && params.contains_key("end") {
    return Ok(Pagination { 
      start: params.get("start").unwrap().parse::<usize>().map_err(ApiError::ParseError)?, 
      end: params.get("end").unwrap().parse::<usize>().map_err(ApiError::ParseError)?
    })
  }

  Err(ApiError::MissingParamError)
}

#[derive(Debug)]
enum ApiError {
  ParseError(std::num::ParseIntError),
  MissingParamError,
  QuestionNotFound
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ApiError::ParseError(ref err) => writeln!(f, "could not parse parameter: {}", err),
      ApiError::MissingParamError => writeln!(f, "missing parameter"), 
      ApiError::QuestionNotFound => writeln!(f, "question not found") 
    }
  }
}

impl Reject for ApiError {}

#[derive(Clone)]
struct Store {
  questions: Arc<RwLock<HashMap<QuestionId, Question>>>
}

impl Store {
  fn new() -> Self {
    Store { 
      questions: Self::init()
    }
  }

  fn init() -> Arc<RwLock<HashMap<QuestionId, Question>>> {
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
  } else if let Some(error) = r.find::<ApiError>() {
    Ok(warp::reply::with_status(error.to_string(), StatusCode::RANGE_NOT_SATISFIABLE))
  } else if let Some(error) = r.find::<BodyDeserializeError>() {
    Ok(warp::reply::with_status(error.to_string(), StatusCode::UNPROCESSABLE_ENTITY))
  } else {
    Ok(warp::reply::with_status("Route not found".to_string(), StatusCode::NOT_FOUND))
  }
}

async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
  if params.len() > 0 {
    let pagination = extract_pagination(params)?;
    let res: Vec<Question> = store.questions.read().values().cloned().collect();
    let res = &res[pagination.start..pagination.end];
    Ok(warp::reply::json(&res))
  } else {
    let res: Vec<Question> = store.questions.read().values().cloned().collect();
    Ok(warp::reply::json(&res))
  }
}

async fn add_question(store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
  store.questions.write().insert(question.id.clone(), question);
  Ok(warp::reply::with_status("Question added", StatusCode::CREATED))
}

async fn update_question(id: String, store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().get_mut(&QuestionId(id)) {
    Some(q) => *q = question,
    None => return Err(warp::reject::custom(ApiError::QuestionNotFound))
  }

  Ok(warp::reply::with_status("question updated", StatusCode::OK))
}

async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().remove(&QuestionId(id)) {
    Some(_) => return Ok(warp::reply::with_status("question removed", StatusCode::OK)),
    None => return Err(warp::reject::custom(ApiError::QuestionNotFound))
  }
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
    .and_then(get_questions);

  let add_question_route = warp::post()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(add_question);

  let update_question_route = warp::put()
    .and(warp::path("questions"))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(update_question);

  let delete_question_route = warp::delete()
    .and(warp::path("questions"))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(delete_question);

  let routes = get_questions_route
    .or(add_question_route)
    .or(update_question_route)
    .or(delete_question_route)
    .with(cors)
    .recover(error_handler);

  println!("Listening on: http://locahost:3333");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
