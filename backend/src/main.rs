#![feature(async_closure)]

use axum::{
  http::StatusCode,
  response::Html,
  routing::{get, get_service},
  Router,
};
use once_cell::sync::Lazy;
// use serde::{Deserialize, Serialize};
use std::{env, include_str, net::SocketAddr, path::Path};
use tower_http::{services::ServeDir, trace::TraceLayer};

static FRONTEND_PATH: Lazy<&Path> = Lazy::new(|| Path::new("../frontend"));

static PORT: Lazy<u16> = Lazy::new(|| {
  env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .unwrap_or(3000)
});

const INDEX: Html<&str> = Html(include_str!("../../frontend/index.html"));

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new()
    // `POST /users` goes to `create_user`
    .nest(
      "/static",
      get_service(ServeDir::new(FRONTEND_PATH.join("static"))).handle_error(
        |error: std::io::Error| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
          )
        },
      ),
    )
    .layer(TraceLayer::new_for_http())
    .nest(
      "/wasm",
      get_service(ServeDir::new(FRONTEND_PATH.join("pkg"))).handle_error(
        |error: std::io::Error| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
          )
        },
      ),
    )
    .layer(TraceLayer::new_for_http())
    .fallback(get(async || INDEX));

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], *PORT));

  tracing::info!("listening on http://{}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
