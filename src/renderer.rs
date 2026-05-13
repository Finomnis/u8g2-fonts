use crate::font_reader::FontReader;

pub mod render_actions;

/// Renders text of a specific [`Font`] to a [`DrawTarget`].
pub type FontRenderer = FontReader;

#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;

    use super::*;

    #[test]
    fn implements_debug() {
        println!(
            "{:?}",
            FontRenderer::new(crate::fonts::u8g2_font_u8glib_4_tf)
        );
    }
}
