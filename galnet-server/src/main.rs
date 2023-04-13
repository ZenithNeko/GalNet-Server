use warp::{Filter, Rejection};
mod handlers;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    println!("GalNet Server Starting..");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
