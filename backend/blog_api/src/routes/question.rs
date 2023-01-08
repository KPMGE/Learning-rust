use error_handler::ApiLayerError;
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing::{instrument, event};
use warp::hyper::StatusCode;
use std::collections::HashMap;
use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{Question, NewQuestion};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiResponse {
  message: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
  original: String,
  word: String,
  deviations: i64,
  info: i64,
  #[serde(rename = "replaceLen")]
  replace_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
  content: String,
  bad_words_total: i64,
  bad_words_list: Vec<BadWord>,
  censored_content: String,
}

#[instrument]
pub async fn get_questions(
  params: HashMap<String, String>,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  event!(target: "blog_api", Level::INFO, "querying questions");
  let mut pagination = Pagination::default();

  if !params.is_empty() {
    event!(Level::INFO, pagination = true);
    pagination = extract_pagination(params)?;
  }

  match store.get_questions(pagination.limit, pagination.offset).await {
    Ok(res) => Ok(warp::reply::json(&res)),
    Err(e) => return Err(warp::reject::custom(e)),
  }
}

async fn transform_error(res: reqwest::Response) -> ApiLayerError {
  ApiLayerError {
    status: res.status().as_u16(),
    message: res.json::<ApiResponse>().await.unwrap().message,
  }
}

pub async fn add_question(
  store: Store,
  new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
  let client = reqwest::Client::new();
  let res = client.post("https://api.apilayer.com/bad_words?censor_character=*")
    .header("apikey", "Some api key")
    .body("a list of shit words")
    .send()
    .await
    .map_err(|e| error_handler::ApiError::ExternalApiError(e))?;


  if !res.status().is_success() {
    if res.status().is_client_error() {
      let err = transform_error(res).await;
      return Err(error_handler::ApiError::ClientError(err).into())
    }

    let err = transform_error(res).await;
    return Err(error_handler::ApiError::ServerError(err).into())
  }

  let res = res.json::<BadWordsResponse>().await.map_err(|e| error_handler::ApiError::ExternalApiError(e))?;

  let question = NewQuestion {
    title: new_question.title,
    content: res.censored_content,
    tags: new_question.tags
  };

  match store.add_question(question).await {
    Ok(question) => Ok(warp::reply::json(&question)),
    Err(e) => Err(warp::reject::custom(e))
  }
}

pub async fn update_question(
  id: i32,
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {    
  match store.update_question(question, id).await {
    Ok(res) => Ok(warp::reply::json(&res)),
    Err(e) => Err(warp::reject::custom(e)),
  }
}

pub async fn delete_question(
  id: i32,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.delete_question(id).await {
    Ok(_) => Ok(warp::reply::with_status(format!("question {} deleted", id), StatusCode::OK)),
    Err(e) => Err(warp::reject::custom(e)),
  }
}
