mod not_found;
mod home;

pub use not_found::NotFound;
pub use home::Home;

use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[not_found]
  #[at("/404")]
  NotFound
}

pub fn switch(route: &Route) -> Html {
  match route {
    Route::NotFound => html! { <NotFound /> },
    Route::Home => html! { <Home /> }
  }
}
