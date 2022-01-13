use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
  #[prop_or_default]
  children: Children,
  #[prop_or_default]
  id: String,
  #[prop_or_default]
  alert_type: String,
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
      <div id={ ctx.props().id.clone() } class={ classes!("alert", {
        let alert_type = ctx.props().alert_type; if alert_type != String::default() {alert_type} } ) }>{ for ctx.props().children.iter() }</div>
    }
  }
}
