/// Error type.
#[derive(Debug)]
pub struct Error {
    /// Error message.
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Error {
            message: message.into(),
        }
    }
}

/// Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Macro to generate an error.
macro_rules! error {
    ( $( $arg:expr ),* ) => {
        Err(Error::new(format!( $( $arg, )* )))
    }
}
