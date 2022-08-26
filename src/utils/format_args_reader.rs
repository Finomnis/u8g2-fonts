use core::fmt::{Arguments, Write};

pub struct FormatArgsReader<F, E> {
    action: F,
    error: Option<E>,
}

impl<F, E> FormatArgsReader<F, E>
where
    F: FnMut(char) -> Result<bool, E>,
{
    pub fn new(action: F) -> Self {
        Self {
            action,
            error: None,
        }
    }

    pub fn process_args(mut self, args: Arguments<'_>) -> Result<(), E> {
        core::fmt::write(&mut self, args).ok();

        match self.error {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl<F, E> Write for FormatArgsReader<F, E>
where
    F: FnMut(char) -> Result<bool, E>,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.error.is_some() {
            return Err(core::fmt::Error);
        }
        for char in s.chars() {
            match (self.action)(char) {
                Ok(true) => (),
                Ok(false) => break,
                Err(e) => {
                    self.error = Some(e);
                    return Err(core::fmt::Error);
                }
            }
        }
        Ok(())
    }
}

pub struct FormatArgsReaderInfallible<F> {
    action: F,
}

impl<F> FormatArgsReaderInfallible<F>
where
    F: FnMut(char),
{
    pub fn new(action: F) -> Self {
        Self { action }
    }

    pub fn process_args(mut self, args: Arguments<'_>) {
        core::fmt::write(&mut self, args).ok();
    }
}

impl<F> Write for FormatArgsReaderInfallible<F>
where
    F: FnMut(char),
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            (self.action)(char);
        }
        Ok(())
    }
}
