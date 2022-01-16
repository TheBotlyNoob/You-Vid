pub mod api;

use once_cell::sync::Lazy;
use prelude::*;
use std::{env, include_str, path::Path};
use tide_compress::CompressMiddleware;
use tide_tracing::TraceMiddleware;

pub static FRONTEND_PATH: Lazy<&Path> = Lazy::new(|| Path::new("../frontend"));

pub static PORT: Lazy<u16> = Lazy::new(|| {
  env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .unwrap_or(3000)
});

pub static DATABASE: Lazy<sled::Db> =
  Lazy::new(|| sled::open("db").expect("Failed to create database dir"));

#[tokio::main]
async fn main() -> TideResult<()> {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "backend=debug,tide=debug")
  }

  // initialize tracing
  tracing_subscriber::fmt::init();

  let mut server = Server::new();

  server.with(TraceMiddleware::new());
  server.with(CompressMiddleware::new());

  server.at("/api").nest(api::router());

  server
    .at("/static")
    .serve_dir(FRONTEND_PATH.join("static"))?;

  server.at("/wasm").serve_dir(FRONTEND_PATH.join("pkg"))?;

  server.at("/*").get(frontend).at("/").get(frontend);

  println!(
    "{}",
    frontend_server::App::new()
      .pdom
      .root_node()
      .dyn_into::<Element>()
  );

  server.listen(format!("0.0.0.0:{}", *PORT)).await?;

  Ok(())
}

pub mod prelude {
  pub use super::DATABASE as database;
  pub use common::prelude::*;
  pub use serde::Deserialize;
  pub use serde_json::{from_value as json_from_value, json};
  pub use tide::{http::mime, Body, Request, Response, Result as TideResult, Server};
  pub use wasm_bindgen::cast::JsCast;
}

async fn frontend(_req: Request<()>) -> TideResult<Response> {
  Ok(
    Response::builder(200)
      .content_type(mime::HTML)
      .body(include_str!("../../frontend/client/index.html"))
      .build(),
  )
}
