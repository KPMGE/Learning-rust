use crate::store::Store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn add_anwer(
  store: Store,
  params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
  let answer = Answer {
    id: AnswerId(String::from("1234")),
    question_id: QuestionId(String::from(params.get("relationId").unwrap())),
    content: String::from(params.get("content").unwrap()),
  };

  // store.answers.write().insert(answer.id.clone(), answer);

  Ok(warp::reply::with_status("answer added", StatusCode::OK))
}
