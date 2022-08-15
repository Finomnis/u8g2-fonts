mod font;
pub mod fonts;
mod renderer;

pub use font::Font;
pub use renderer::FontRenderer;

pub const fn create_font_renderer<F: Font>() -> FontRenderer {
    FontRenderer::new::<F>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
