use core::fmt::{Debug, Display};

/// The error types.
#[derive(Debug)]
pub enum Error<DisplayError> {
    /// Font does not support background color
    BackgroundColorNotSupported,
    /// Font does not contain given character
    GlyphNotFound(char),
    /// Internal error
    InternalError,
    /// Writing to display failed
    DisplayError(DisplayError),
}

impl<DisplayError> Display for Error<DisplayError>
where
    DisplayError: Display + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
            Error::DisplayError(e) => write!(f, "Writing to display failed: {e}"),
        }
    }
}
