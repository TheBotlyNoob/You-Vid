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
  pub use common::*;
  pub use yew::prelude::*;
  pub use yew_router::prelude::*;
}
