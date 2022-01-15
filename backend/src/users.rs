use crate::prelude::*;

pub async fn login(
  Json(payload): Json<api::users::login::Request>,
) -> Json<api::users::login::Response> {
  database.insert(payload.email, payload.password);

  Json(
    api::users::login::Response::deserialize(&json!({
      "": ""
    }))
    .unwrap(),
  )
}

pub fn router() -> Router {
  Router::new().route("/login", post(login))
}
