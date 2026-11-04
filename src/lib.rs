//! # Implements Deserialize
//!
//! ```
//! use twitch_surf::SubscriptionType;
//!
//! pub struct Subscription {
//!     kind: SubscriptionType,
//!     version: String,
//! }
//!
//! impl<'de> serde::Deserialize<'de> for Subscription {
//!     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//!     where
//!         D: serde::Deserializer<'de>,
//!     {
//!         #[derive(serde::Deserialize)]
//!         struct Helper {
//!             #[serde(rename = "type")]
//!             kind: String,
//!             version: String,
//!         }
//!
//!         let helper = Helper::deserialize(deserializer)?;
//!         let ty = SubscriptionType::parse(&helper.kind, &helper.version)
//!             .map_err(serde::de::Error::custom)?;
//!
//!         Ok(Subscription {
//!             kind: ty,
//!             version: ty.version().to_owned(),
//!         })
//!     }
//! }
//! ```
mod subscription_type;

pub use subscription_type::{ParseSubscriptionTypeError, SubscriptionType};
