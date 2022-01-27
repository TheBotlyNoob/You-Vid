use prelude::*;

#[derive(rust_embed::RustEmbed)]
#[folder = "$OUT_DIR/ui/static"]
struct ClientDir;

const INDEX: &str = std::include_str!(concat!(env!("OUT_DIR"), "/ui/index.html"));

#[tokio::main]
async fn main() {
  let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".into())
    .parse::<u16>()
    .unwrap_or_else(|_| {
      eprintln!("failed to parse $PORT, using default");
      3000
    });

  // We'll bind to 127.0.0.1:$PORT
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  // A `Service` is needed for every connection, so this
  // creates one from our `hello_world` function.
  let make_svc = make_service_fn(|_conn| async {
    // service_fn converts our function into a `Service`
    Ok::<_, Infallible>(service_fn(router))
  });

  println!("listening on http://127.0.0.1:{port}");
  let server = Server::bind(&addr).serve(make_svc);

  // Run this server for... forever!
  if let Err(err) = server.await {
    eprintln!("server error: {err}");
  }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let mut res = Response::new(Body::empty());

  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      *res.body_mut() = Body::from(INDEX.replacen("__INITIAL_HTML__", "", 1).replacen(
        "__INITIAL_STATE__",
        "",
        1,
      ));
      res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("text/html").unwrap(),
      );
    }
    (&Method::POST, "/echo") => {
      // we'll be back
    }
    _ if req.uri().path().starts_with("/api") => {
      *res.status_mut() = StatusCode::NOT_FOUND;
      *res.body_mut() = Body::from("");
    }
    _ => {
      *res.status_mut() = StatusCode::NOT_FOUND;
    }
  };

  Ok(res)
}

pub mod prelude {
  pub use futures::{Future, FutureExt, TryFuture, TryFutureExt};
  pub use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
  };
  pub use std::{convert::Infallible, env, net::SocketAddr};
}
