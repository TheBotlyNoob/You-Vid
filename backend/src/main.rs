#![feature(async_closure)]

use axum::{
  http::StatusCode,
  routing::{get, get_service},
  Router,
  response::Html
};
use once_cell::sync::Lazy;
use std::{env, include_str, io::Error as IoError, net::SocketAddr, path::Path};
use tower_http::{services::ServeDir, trace::TraceLayer};


static FRONTEND_PATH: Lazy<&Path> = Lazy::new(|| Path::new("../../frontend"));

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
    .route(
      "/static",
      get_service(ServeDir::new(FRONTEND_PATH.join("static"))).handle_error(
        |error: IoError| async move {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error)
          )
        }
      )
    )
    .layer(TraceLayer::new_for_http())
    .route(
      "/wasm",
      get_service(ServeDir::new(FRONTEND_PATH.join("pkg"))).handle_error(|error: IoError| async move {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Unhandled internal error: {}", error)
        )
      })
    );

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u16>().expect("Failed to parse port")));

  tracing::debug!("listening on http://{}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
