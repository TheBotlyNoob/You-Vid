pub mod api;

pub type Base64 = String;
pub type UUID = String;

pub mod prelude {
  pub type Base64 = super::Base64;
  pub type UUID = super::UUID;

  pub type Error = super::api::error::Error;

  pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
