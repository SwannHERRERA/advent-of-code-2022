pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
pub struct Wrapper<T>(pub T);

pub use std::format as f;
