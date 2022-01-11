#![feature(async_closure)]
use axum::{
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::{get, get_service},
  Json, Router
};
use serde::{Deserialize, Serialize};
use std::{env, io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};

const INDEX: Html<&str> = Html(
  r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"/><meta http-equiv="X-UA-Compatible" content="IE=edge"/><meta name="viewport" content="width=device-width,initial-scale=1.0"/><title>You-Vid</title><script type="module">import init,{run_app}from '/pkg/you_vid.js';init('/wasm/you_vid_bg.wasm').then(run_app);</script></head><body></body></html>"#
);

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  if env::var_os("RUST_LOG").is_none() {
    env::set_var("RUST_LOG", "you_vid=debug");
  }

  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(async || INDEX))
    .route(
      "/static",
      get_service(ServeDir::new("static")).handle_error(|error: io::Error| async move {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Unhandled internal error: {}", error)
        )
      })
    )
    .layer(TraceLayer::new_for_http())
    .route(
      "/wasm",
      get_service(ServeDir::new("pkg")).handle_error(|error: io::Error| async move {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Unhandled internal error: {}", error)
        )
      })
    );

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  tracing::debug!("listening on http://{}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn create_user(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Json(payload): Json<CreateUser>
) -> impl IntoResponse {
  // insert your application logic here
  let user = User {
    id: 1337,
    username: payload.username
  };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
  username: String
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
  id: u64,
  username: String
}
