#[macro_use]
extern crate diesel_derive_newtype;

pub mod schema;
pub mod user;
pub mod session;

#[cfg(test)]
pub mod test_db;

pub use diesel::result::Error as DieselError;

pub mod error;
pub use error::QueryError;

pub mod util;
pub use util::{AsyncConnection, AsyncConnectionPool, OwnedAsyncConnection};

use socialverse_domain::ids::*;