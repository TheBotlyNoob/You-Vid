use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Error {
  pub message: String,
  pub location: String,
}
