use core::fmt::Arguments;

use super::Content;

pub struct ArgsContent<'a>(pub Arguments<'a>);

impl<'a> Content for ArgsContent<'a> {}
