use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  id: String,
  #[prop_or_default]
  pub alert_type: String,
}

pub struct Alert;

impl Component for Alert {
  type Message = ();
  type Properties = AlertProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div
        id={ ctx.props().id.clone() }
        role="alert"
        class={ classes!("alert", format!("alert-{}", if ctx.props().alert_type == String::default() {
                                            "primary".to_string()
                                          } else {
                                            ctx.props().alert_type.clone()
                                          }
                                        )) }>{ for ctx.props().children.iter() }</div>
    }
  }
}
