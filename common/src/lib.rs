#![feature(type_alias_impl_trait)]
pub mod api;

pub type Base64 = String;
pub type Base64Ref<'a> = &'a str;
pub type UUID = String;

pub mod prelude {
  pub type Base64 = super::Base64;
  pub type UUID = super::UUID;

  pub type ApiError = super::api::error::Error;

  pub use super::api;

  pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
