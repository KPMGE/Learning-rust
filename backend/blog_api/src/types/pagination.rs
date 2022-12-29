use std::collections::HashMap;
use error_handler::ApiError;

#[derive(Debug)]
pub struct Pagination {
  pub start: usize,
  pub end: usize
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, ApiError> {
  if params.contains_key("start") && params.contains_key("end") {
    return Ok(Pagination { 
      start: params.get("start").unwrap().parse::<usize>().map_err(ApiError::ParseError)?, 
      end: params.get("end").unwrap().parse::<usize>().map_err(ApiError::ParseError)?
    })
  }

  Err(ApiError::MissingParamError)
}
