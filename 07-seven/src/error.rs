#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl<T> From<nom::Err<T>> for Error {
    fn from(_: nom::Err<T>) -> Self {
        Error::Generic("Parsing error".to_string())
    }
}
