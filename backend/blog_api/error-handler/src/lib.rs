use warp::{Rejection, Reply};
use warp::reject::Reject;
use warp::cors::CorsForbidden;
use warp::body::BodyDeserializeError;
use warp::hyper::StatusCode;
use std::fmt;
use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum ApiError {
  ParseError(std::num::ParseIntError),
  MissingParamError,
  QuestionNotFound,
  DatabaseQueryError(SqlxError),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &*self {
      ApiError::ParseError(ref err) => writeln!(f, "could not parse parameter: {}", err),
      ApiError::MissingParamError => writeln!(f, "missing parameter"), 
      ApiError::QuestionNotFound => writeln!(f, "question not found"),
      ApiError::DatabaseQueryError(err) => write!(f, "query could not be executed: {}", err)
    }
  }
}

impl Reject for ApiError {}

pub async fn handle_errors(r: Rejection) -> Result<impl Reply, Rejection> {
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
