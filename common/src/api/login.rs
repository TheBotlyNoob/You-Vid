use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginRequest {
  password: String,
  email: String,
  profile_picture: Option<Base64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginResponse {
  pub user_id: u32,
  pub user_name: String,
  pub profile_picture: Base64,
}
