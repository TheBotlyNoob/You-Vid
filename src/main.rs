#![cfg_attr(not(debug_assertions), deny(warnings))]

#[tokio::main]
async fn main() {
  println!("Hello, world!");
}

pub mod server_prelude {
  pub use tide::prelude::*;
  pub use tide::Request as TideRequest;
}

pub mod client_prelude {
  pub use percy_dom::prelude::*;
}
