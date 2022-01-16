mod prelude {
  pub use common::prelude::*;
  pub use gloo::console;
  pub use percy_dom::prelude::*;
  pub use std::error::Error;
  pub use wasm_bindgen::{prelude::*, JsCast};
  pub use wasm_bindgen_futures::{spawn_local, JsFuture};
  pub use web_sys::Response;
}
