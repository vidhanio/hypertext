use std::net::Ipv4Addr;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{handle_about, handle_home, handle_list};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod handlers;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handle_home))
        .route("/about", get(handle_about))
        .route("/list", get(handle_list))
        .fallback_service(ServeDir::new("static"));

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
