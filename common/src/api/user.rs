use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LoginRequest {
  pub password: String,
  pub email: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LoginResponse<User> {
  pub user: User,
}
