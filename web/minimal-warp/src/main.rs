// Minimal http server with warp crate

use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("hello {}!", name));

    println!("serving at: localhost:3000...");
    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}
