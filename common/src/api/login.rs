use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginRequest {
  password: String,
  email: String
}
