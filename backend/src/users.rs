use crate::prelude::*;

pub async fn login(
  Json(payload): Json<api::users::login::Request>,
) -> Json<api::users::login::Response> {
  Json(api::users::login::Response {
    error: None,
    user_id: 1,
    profile_picture: payload
      .profile_picture
      .unwrap_or_else(|| api::users::DEFAULT_PFP.to_string()),
    email: payload.email,
    user_name: "Hello, There".to_string(),
  })
}

pub fn router() -> Router {
  Router::new().route("/login", post(login))
}
