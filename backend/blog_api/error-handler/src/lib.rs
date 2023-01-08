use warp::{Rejection, Reply};
use warp::reject::Reject;
use warp::cors::CorsForbidden;
use warp::body::BodyDeserializeError;
use warp::hyper::StatusCode;
use reqwest::Error as ReqwestError;

use tracing::{event, Level};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ApiLayerError {
  pub status: u16,
  pub message: String
}

impl std::fmt::Display for ApiLayerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Status: {}, Message: {}", self.status, self.message)
  }
}

#[derive(Debug)]
pub enum ApiError {
  ParseError(std::num::ParseIntError),
  MissingParamError,
  DatabaseQueryError,
  ExternalApiError(ReqwestError),
  ClientError(ApiLayerError),
  ServerError(ApiLayerError),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &*self {
      ApiError::ParseError(ref err) => writeln!(f, "could not parse parameter: {}", err),
      ApiError::MissingParamError => writeln!(f, "missing parameter"), 
      ApiError::DatabaseQueryError => write!(f, "cannot update, invalid data."),
      ApiError::ExternalApiError(err) => write!(f, "cannot execute: {}", err),
      ApiError::ClientError(err) => write!(f, "external client error: {}", err),
      ApiError::ServerError(err) => write!(f, "external server error: {}", err),
    }
  }
}

impl Reject for ApiError {}
impl Reject for ApiLayerError {}

pub async fn handle_errors(r: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(crate::ApiError::DatabaseQueryError) = r.find() {
    event!(Level::ERROR, "database query error");
    Ok(warp::reply::with_status(
      crate::ApiError::DatabaseQueryError.to_string(), 
      StatusCode::UNPROCESSABLE_ENTITY
    ))
  } else if let Some(error) = r.find::<CorsForbidden>() {
    event!(Level::ERROR, "CORS forbidden error: {}", error);
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::FORBIDDEN
    ))
  } else if let Some(error) = r.find::<ApiError>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::RANGE_NOT_SATISFIABLE
    ))
  } else if let Some(error) = r.find::<BodyDeserializeError>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::UNPROCESSABLE_ENTITY
    ))
  } else if let Some(crate::ApiError::ExternalApiError(e)) = r.find() {
    event!(Level::ERROR, "external api error: {}", e);
    Ok(warp::reply::with_status(
      "Internal server error".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR
    ))
  } else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND
    ))
  }
}
