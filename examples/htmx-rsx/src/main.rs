use axum::{Router, routing::get};
use handlers::{handle_about, handle_home, handle_list};
use tower_http::services::ServeDir;

mod handlers;
mod views;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handle_home))
        .route("/about", get(handle_about))
        .route("/list", get(handle_list))
        .nest_service("/static", ServeDir::new("static"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
