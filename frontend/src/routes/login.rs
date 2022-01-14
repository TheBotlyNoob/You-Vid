use crate::prelude::*;
use gloo::utils::window;

#[derive(Default, Clone)]
pub struct Login {
  error: Option<String>,
}

pub enum Msg {
  Login(String, String),
  SetError(Option<String>),
  Nothing,
}

impl Component for Login {
  type Message = Msg;
  type Properties = ();

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    let link = ctx.link();

    match msg {
      Msg::Login(_email, _password) => {
        link.send_future(async {
          let res = JsFuture::from(
            window()
              .fetch_with_str_and_init("/api/login", web_sys::RequestInit::new().method("POST")),
          )
          .await;

          console::error!(format!("Hello {:#?}", res));

          if let Err(error) = res {
            return Msg::SetError(error.as_string());
          }

          let res = res.unwrap().dyn_into::<Response>().unwrap();

          if !res.ok() {
            return Msg::SetError(Some(format!("Got status code {}", res.status())));
          }

          Msg::Nothing
        });

        true
      }

      Msg::SetError(error) => {
        self.error = error;

        true
      }

      Msg::Nothing => false,
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
