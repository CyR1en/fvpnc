//! crate prelude

use std::fs::write;
pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
