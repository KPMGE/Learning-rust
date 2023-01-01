#![warn(clippy::all)]

use tracing_subscriber::fmt::format::FmtSpan;
use warp::hyper::Method;
use warp::Filter;

mod routes;
mod store;
mod types;

use error_handler::handle_errors;
use routes::answer::*;
use routes::question::*;
use store::Store;

#[tokio::main]
async fn main() {
  let store = Store::new();
  let store_filter = warp::any().map(move || store.clone());
  let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "blog_api=info,warp=error".to_owned());

  tracing_subscriber::fmt()
    .with_env_filter(log_filter)
    .with_span_events(FmtSpan::CLOSE)
    .init();

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::DELETE]);

  let get_questions_route = warp::get()
    .and(warp::path("questions"))
    .and(warp::query())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(get_questions)
    .with(warp::trace(|info| {
      tracing::info_span!(
        "get_questions request",
        method = %info.method(),
        path = %info.path(), 
      )
    }));

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

  let add_anwer_route = warp::post()
    .and(warp::path("answers"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::form())
    .and_then(add_anwer);

  let routes = get_questions_route
    .or(add_question_route)
    .or(update_question_route)
    .or(delete_question_route)
    .or(add_anwer_route)
    .with(cors)
    .with(warp::trace::request())
    .recover(handle_errors);

  println!("Listening on: http://locahost:3333...");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
