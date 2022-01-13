use crate::prelude::*;
use gloo_console as console;

///Define the possible messages which can be sent to the component
#[derive(PartialEq, Debug)]
pub enum Msg {
  Increment,
  Decrement,
}

#[derive(PartialEq, Debug, Default)]
pub struct Home {
  value: i64, // This will store the counter value
}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self::default()
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
    match msg {
      Msg::Increment => {
        self.value += 1;
        console::log!("plus one"); // Will output a string to the browser console
        true // Return true to cause the displayed change to update
      }
      Msg::Decrement => {
        self.value -= 1;
        console::log!("minus one");
        true
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <h3>{ "All Uploaded Videos" }</h3>
    }
  }
}
