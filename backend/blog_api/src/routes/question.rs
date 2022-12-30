use error_handler::ApiError;
use tracing::Level;
use tracing::{instrument, event};
use std::collections::HashMap;
use warp::hyper::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};

#[instrument]
pub async fn get_questions(
  params: HashMap<String, String>,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  event!(target: "blog_api", Level::INFO, "querying questions");

  if !params.is_empty() {
    let pagination = extract_pagination(params)?;

    event!(Level::INFO, pagination = true);

    let res: Vec<Question> = store.questions.read().values().cloned().collect();
    let res = &res[pagination.start..pagination.end];
    Ok(warp::reply::json(&res))
  } else {
    event!(Level::INFO, pagination = false);

    let res: Vec<Question> = store.questions.read().values().cloned().collect();
    Ok(warp::reply::json(&res))
  }
}

pub async fn add_question(
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  store
    .questions
    .write()
    .insert(question.id.clone(), question);
  Ok(warp::reply::with_status(
    "Question added",
    StatusCode::CREATED,
  ))
}

pub async fn update_question(
  id: String,
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().get_mut(&QuestionId(id)) {
    Some(q) => *q = question,
    None => return Err(warp::reject::custom(ApiError::QuestionNotFound)),
  }

  Ok(warp::reply::with_status("question updated", StatusCode::OK))
}

pub async fn delete_question(
  id: String,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().remove(&QuestionId(id)) {
    Some(_) => Ok(warp::reply::with_status("question removed", StatusCode::OK)),
    None => Err(warp::reject::custom(ApiError::QuestionNotFound)),
  }
}
