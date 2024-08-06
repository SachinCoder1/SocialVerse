#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod user;
pub mod ids;

pub use user::{Password,Username};