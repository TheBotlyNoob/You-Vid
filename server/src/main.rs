#![cfg_attr(not(debug_assertions), deny(warnings))]

#[tokio::main]
async fn main() {
  println!("{}", env!("OUT_DIR"));
}

pub mod server_prelude {
  pub use tide::prelude::*;
  pub use tide::Request as TideRequest;
}
