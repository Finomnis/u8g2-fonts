use core::fmt::Arguments;

use crate::utils::{FormatArgsReader, FormatArgsReaderInfallible};

use super::Content;

pub struct ArgsContent<'a>(pub Arguments<'a>);

impl<'a> Content for ArgsContent<'a> {
    fn for_each_char<F, E>(&self, func: F) -> Result<(), E>
    where
        F: FnMut(char) -> Result<(), E>,
    {
        FormatArgsReader::new(func).process_args(self.0)
    }

    fn for_each_char_infallible<F>(&self, func: F)
    where
        F: FnMut(char),
    {
        FormatArgsReaderInfallible::new(func).process_args(self.0)
    }
}
