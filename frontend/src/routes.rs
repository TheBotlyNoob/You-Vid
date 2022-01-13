mod not_found;
pub use not_found::*;

mod home;
pub use home::*;

mod login;
pub use login::*;

use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/login")]
  Login,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: &Route) -> Html {
  match route {
    Route::NotFound => html! { <NotFound /> },
    Route::Home => html! { <Home /> },
    Route::Login => html! { <Login /> },
  }
}
