#[derive(Debug)]
pub enum Error {
    BACKGROUND_COLOR_NOT_SUPPORTED,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Error::BACKGROUND_COLOR_NOT_SUPPORTED => {
                "This font does not support a background color."
            }
        })
    }
}
