pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
pub struct Wrapper<T>(pub T);

pub fn f(s: &str) -> String {
    String::from(s)
}
