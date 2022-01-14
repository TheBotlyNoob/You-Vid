use crate::prelude::*;
use gloo::utils::window;

#[derive(Default)]
pub struct Login {
  error: Option<String>,
}

pub enum Msg {
  Login(String, String),
}

impl Component for Login {
  type Message = Msg;
  type Properties = ();

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Login(_email, _password) => {
        let res = window()
          .fetch_with_str_and_init("/api/login", web_sys::RequestInit::new().method("POST"))
          .then(&Closure::wrap(Box::new(|res: JsValue| {
            let res = res.dyn_into::<Response>().unwrap();
          }) as Box<dyn FnMut(JsValue)>))
          .catch(&Closure::wrap(Box::new(|err: JsValue| {
            self.error = err.as_string();
          }) as Box<dyn FnMut(JsValue)>>);

        true
      }
    }
  }

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      error: Some("An Error".to_string()),
      // ..Self::default()
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();

    html! {
      <>
        if let Some(error) = &self.error {
          <Alert alert_type="danger">{ error }</Alert>
        }

        <button onclick={ link.callback(|_| Msg::Login(String::new(), String::new())) }></button>
      </>
    }
  }
}
