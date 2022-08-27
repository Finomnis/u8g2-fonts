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
        for char in s.chars() {
            match (self.action)(char) {
                Ok(true) => (),
                Ok(false) => {
                    // Returning an error here doesn't
                    // actually cause `process_args` to error,
                    // it just stops the `write` call.
                    return Err(core::fmt::Error);
                }
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

#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::String;

    use super::*;

    #[test]
    fn format_args_reader_provides_values() {
        let mut output = String::new();

        FormatArgsReader::new(|ch| -> Result<bool, &'static str> {
            output.push(ch);
            Ok(true)
        })
        .process_args(format_args!("Hello, w{}rld!", 0))
        .unwrap();

        assert_eq!(output, "Hello, w0rld!");
    }

    #[test]
    fn format_args_reader_forwards_errors() {
        let result = FormatArgsReader::new(|_| -> Result<bool, &'static str> { Err("Error!") })
            .process_args(format_args!("Hello, w{}rld!", 0));

        assert!(matches!(result, Err("Error!")));
    }

    #[test]
    fn format_args_reader_stops_when_requested() {
        let mut output = String::new();

        FormatArgsReader::new(|ch| -> Result<bool, &'static str> {
            output.push(ch);
            Ok(output.len() < 5)
        })
        .process_args(format_args!("Hello, w{}rld!", 0))
        .unwrap();

        assert_eq!(output, "Hello");
    }

    #[test]
    fn infallible_format_args_reader_provides_values() {
        let mut output = String::new();

        FormatArgsReaderInfallible::new(|ch| {
            output.push(ch);
        })
        .process_args(format_args!("Hello, w{}rld!", 0));

        assert_eq!(output, "Hello, w0rld!");
    }
}
