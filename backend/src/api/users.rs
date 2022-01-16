use crate::prelude::*;

pub async fn login(mut req: Request<()>) -> TideResult<Body> {
  let payload = req.body_json::<api::users::login::Request>().await?;

  Body::from_json(&api::users::login::Response {
    error: None,
    user_id: 1,
    profile_picture: payload
      .profile_picture
      .unwrap_or_else(|| api::users::DEFAULT_PFP.to_string()),
    email: payload.email,
    user_name: "Hello, There".to_string(),
  })
}

pub fn router() -> Server<()> {
  let mut users = Server::new();

  users.at("/api/users/login").post(login);

  users
}
