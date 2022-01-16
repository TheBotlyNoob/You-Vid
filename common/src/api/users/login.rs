use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Request {
  pub password: String,
  pub email: String,
  pub profile_picture: Option<Base64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Response {
  pub error: Option<ApiError>,
  pub user_id: u32,
  pub user_name: String,
  pub profile_picture: Base64,
  pub email: String,
}
