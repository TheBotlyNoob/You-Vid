pub mod components;
pub mod routes;

use prelude::*;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
  yew::start_app::<App>();

  Ok(())
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <main id="root">
        <Switch<routes::Route> render={Switch::render(routes::switch)} />
      </main>
    </BrowserRouter>
  }
}

mod prelude {
  pub use super::components::*;
  pub use common::prelude::*;
  pub use gloo::console;
  pub use std::{
    error::Error,
    sync::{Arc, Mutex},
  };
  pub use wasm_bindgen::{prelude::*, JsCast};
  pub use wasm_bindgen_futures::{spawn_local, JsFuture};
  pub use web_sys::Response;
  pub use yew::prelude::*;
  pub use yew_router::prelude::*;
}
