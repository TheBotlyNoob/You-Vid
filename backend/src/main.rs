#![feature(async_closure)]

use axum::{
  http::StatusCode,
  response::Html,
  routing::{get, get_service},
  Router
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
    .expect("Failed to parse port")
});

const INDEX: Html<&str> = Html(include_str!("../../frontend/index.html"));

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "you_vid_backend=debug");
  }

  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(async || INDEX))
    // `POST /users` goes to `create_user`
    .nest(
      "/static",
      get_service(ServeDir::new(FRONTEND_PATH.join("static"))).handle_error(
        |error: std::io::Error| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error)
          )
        }
      )
    )
    .layer(TraceLayer::new_for_http())
    .nest(
      "/wasm",
      get_service(ServeDir::new(FRONTEND_PATH.join("pkg"))).handle_error(
        |error: std::io::Error| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error)
          )
        }
      )
    )
    .layer(TraceLayer::new_for_http());

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], *PORT));

  tracing::debug!("listening on {}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
