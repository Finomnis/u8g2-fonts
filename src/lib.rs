mod font;
mod renderer;

pub mod fonts;

pub use font::Font;
pub use renderer::create_font_renderer;
pub use renderer::FontRenderer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
