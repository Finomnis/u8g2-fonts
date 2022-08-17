#[derive(Debug)]
pub enum Error {
    /// Font does not support background color
    BackgroundColorNotSupported,
    /// Font does not contain given character
    GlyphNotFound(char),
    /// Internal error
    InternalError,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BackgroundColorNotSupported => {
                write!(f, "This font does not support a background color.")
            }
            Error::GlyphNotFound(c) => {
                write!(f, "This font does not support the character '{}'.", c)
            }
            Error::InternalError => {
                write!(f, "Internal error.")
            }
        }
    }
}
