use error_handler::ApiError;
use std::collections::HashMap;

/// pagination structure, got from query params
#[derive(Debug)]
pub struct Pagination {
  /// the index of the first item in the range
  pub start: usize,
  /// the index of the last item in the range
  pub end: usize,
}

/// extract query params for the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we can set a range like: 
/// `/questions?start=1&end=10`
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, ApiError> {
  if params.contains_key("start") && params.contains_key("end") {
    return Ok(Pagination {
      start: params
        .get("start")
        .unwrap()
        .parse::<usize>()
        .map_err(ApiError::ParseError)?,
      end: params
        .get("end")
        .unwrap()
        .parse::<usize>()
        .map_err(ApiError::ParseError)?,
    });
  }

  Err(ApiError::MissingParamError)
}
