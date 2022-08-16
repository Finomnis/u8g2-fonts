#[derive(Debug)]
pub enum Error {
    /// Font does not support background color
    BACKGROUND_COLOR_NOT_SUPPORTED,
    /// Font does not contain given character
    GLYPH_NOT_FOUND(char),
    /// Internal error
    INTERNAL_ERROR,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BACKGROUND_COLOR_NOT_SUPPORTED => {
                write!(f, "This font does not support a background color.")
            }
            Error::GLYPH_NOT_FOUND(c) => {
                write!(f, "This font does not support the character '{}'.", c)
            }
            Error::INTERNAL_ERROR => {
                write!(f, "Internal error.")
            }
        }
    }
}
