use core::fmt::{Debug, Display};

/// All possible errors a non-draw call can cause.
#[derive(Debug)]
pub enum LookupError {
    /// Font does not contain given character.
    GlyphNotFound(char),
    /// Internal error.
    InternalError,
}

/// All possible errors a draw call can cause.
#[derive(Debug)]
pub enum Error<DisplayError> {
    /// Font does not contain given character.
    GlyphNotFound(char),
    /// Internal error.
    InternalError,
    /// Writing to display failed.
    DisplayError(DisplayError),
}

impl<DisplayError> Display for Error<DisplayError>
where
    DisplayError: Display + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
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

impl Display for LookupError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            LookupError::GlyphNotFound(c) => {
                write!(f, "This font does not support the character '{}'.", c)
            }
            LookupError::InternalError => {
                write!(f, "Internal error.")
            }
        }
    }
}

impl<T> From<LookupError> for Error<T> {
    fn from(e: LookupError) -> Self {
        match e {
            LookupError::GlyphNotFound(g) => Error::GlyphNotFound(g),
            LookupError::InternalError => Error::InternalError,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LookupError {}

#[cfg(feature = "std")]
impl<DisplayError> std::error::Error for Error<DisplayError> where
    DisplayError: core::fmt::Debug + core::fmt::Display
{
}
