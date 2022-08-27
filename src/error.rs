use core::fmt::{Debug, Display};

/// All possible errors a non-draw call can cause.
#[derive(Debug)]
pub enum LookupError {
    /// Font does not contain given character.
    GlyphNotFound(char),
}

/// All possible errors a draw call can cause.
#[derive(Debug)]
pub enum Error<DisplayError> {
    /// Font does not support a background color.
    BackgroundColorNotSupported,
    /// Font does not contain given character.
    GlyphNotFound(char),
    /// Writing to display failed.
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
        }
    }
}

impl<T> From<LookupError> for Error<T> {
    fn from(e: LookupError) -> Self {
        match e {
            LookupError::GlyphNotFound(g) => Error::GlyphNotFound(g),
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

#[cfg(test)]
mod tests {
    extern crate std;
    use std::{format, println};

    use super::*;

    fn examine_error<T: core::fmt::Display + core::fmt::Debug>(error: T, msg: &str) {
        assert_eq!(format!("{}", error), msg);
        println!("{:?}", error);
    }

    #[test]
    fn errors_are_display_and_debug() {
        examine_error(
            LookupError::GlyphNotFound('a'),
            "This font does not support the character 'a'.",
        );
        examine_error(
            Error::<&'static str>::GlyphNotFound('b'),
            "This font does not support the character 'b'.",
        );
        examine_error(
            Error::<&'static str>::BackgroundColorNotSupported,
            "This font does not support a background color.",
        );
        examine_error(
            Error::<&'static str>::DisplayError("This is a display error!"),
            "Writing to display failed: This is a display error!",
        );
    }
}
