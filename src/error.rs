use core::fmt::{Debug, Display};

/// All possible errors a non-draw call can cause.
#[derive(Debug)]
pub enum Error {
    /// Font does not contain given character.
    GlyphNotFound(char),
    /// Internal error.
    InternalError,
}

/// All possible errors a draw call can cause.
#[derive(Debug)]
pub enum DrawError<DisplayError> {
    /// Font does not support a background color.
    BackgroundColorNotSupported,
    /// Font does not contain given character.
    GlyphNotFound(char),
    /// Internal error.
    InternalError,
    /// Writing to display failed.
    DisplayError(DisplayError),
}

impl<DisplayError> Display for DrawError<DisplayError>
where
    DisplayError: Display + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            DrawError::BackgroundColorNotSupported => {
                write!(f, "This font does not support a background color.")
            }
            DrawError::GlyphNotFound(c) => {
                write!(f, "This font does not support the character '{}'.", c)
            }
            DrawError::InternalError => {
                write!(f, "Internal error.")
            }
            DrawError::DisplayError(e) => write!(f, "Writing to display failed: {e}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::GlyphNotFound(c) => {
                write!(f, "This font does not support the character '{}'.", c)
            }
            Error::InternalError => {
                write!(f, "Internal error.")
            }
        }
    }
}

impl<T> From<Error> for DrawError<T> {
    fn from(e: Error) -> Self {
        match e {
            Error::GlyphNotFound(g) => DrawError::GlyphNotFound(g),
            Error::InternalError => DrawError::InternalError,
        }
    }
}
