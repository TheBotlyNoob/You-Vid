use crate::prelude::*;

#[derive(Default)]
pub struct Login {
  error: Option<String>,
}

impl Component for Login {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      error: Some("An Error".to_string()),
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      if let Some(error) = &self.error {
        <Alert alert_type="danger">{ error }</Alert>
      }
    }
  }
}
