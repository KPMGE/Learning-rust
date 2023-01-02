use error_handler::ApiError;
use std::collections::HashMap;

/// pagination structure, got from query params
#[derive(Debug)]
pub struct Pagination {
  /// the index of the first item in the range
  pub limit: Option<i32>,
  /// the index of the last item in the range
  pub offset: i32,
}

impl Default for Pagination {
  fn default() -> Self {
    Pagination { limit: None, offset: 0 }
  }
}

/// extract query params for the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we can set a range like: 
/// `/questions?start=1&end=10`
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, ApiError> {
  if params.contains_key("limit") && params.contains_key("offset") {
    return Ok(Pagination {
      limit: Some(params
        .get("limit")
        .unwrap()
        .parse()
        .map_err(ApiError::ParseError)?
        ),
      offset: params
        .get("offset")
        .unwrap()
        .parse()
        .map_err(ApiError::ParseError)?,
    });
  }

  Err(ApiError::MissingParamError)
}
